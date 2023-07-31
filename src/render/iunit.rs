use super::{texture::IUnitTextureResource, CameraResource, DepthResource};
use crate::service::Service;
use glam::*;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    texcoord: [f32; 2],
}

impl Vertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRIBUTES,
        }
    }
}

pub struct IUnitPipeline {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    texture_resource: IUnitTextureResource,
    pipeline: wgpu::RenderPipeline,
}

impl IUnitPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        camera_resource: &CameraResource,
    ) -> Self {
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: device.limits().max_buffer_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: device.limits().max_buffer_size,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let texture_resource = IUnitTextureResource::new(device, queue);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                camera_resource.bind_group_layout(),
                texture_resource.bind_group_layout(),
            ],
            push_constant_ranges: &[],
        });

        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../../assets/shaders/unit.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DepthResource::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            vertex_buffer,
            vertex_count: 0,
            index_buffer,
            index_count: 0,
            texture_resource,
            pipeline,
        }
    }

    pub fn pre_draw(&mut self, queue: &wgpu::Queue, service: &Service) {
        if let Some(camera) = service.camera.get_camera() {
            let view_aabb = camera.view_aabb();

            let mut vertices = vec![];
            let mut indices = vec![];

            service
                .iunit
                .get_iunits(view_aabb.as_iaabb3())
                .into_iter()
                .for_each(|iunit| {
                    let origin = iunit.position.as_vec3a();
                    let texcoord_aabb = self
                        .texture_resource
                        .get_texcoord(&iunit.kind)
                        .unwrap_or_else(|| panic!("not registered a iunit kind {:?}", &iunit.kind));

                    let vertex_count = vertices.len() as u32;

                    let position = (origin + Vec3A::new(-0.5, -0.5, -0.5)).into();
                    let texcoord = texcoord_aabb.xw().into();
                    vertices.push(Vertex { position, texcoord });

                    let position = (origin + Vec3A::new(0.5, -0.5, -0.5)).into();
                    let texcoord = texcoord_aabb.zw().into();
                    vertices.push(Vertex { position, texcoord });

                    let position = (origin + Vec3A::new(0.5, 0.5, 0.5)).into();
                    let texcoord = texcoord_aabb.zy().into();
                    vertices.push(Vertex { position, texcoord });

                    let position = (origin + Vec3A::new(-0.5, 0.5, 0.5)).into();
                    let texcoord = texcoord_aabb.xy().into();
                    vertices.push(Vertex { position, texcoord });

                    indices.push(vertex_count + 0);
                    indices.push(vertex_count + 1);
                    indices.push(vertex_count + 2);
                    indices.push(vertex_count + 2);
                    indices.push(vertex_count + 3);
                    indices.push(vertex_count + 0);
                });

            self.vertex_count = vertices.len() as u32;
            self.index_count = indices.len() as u32;
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
            queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));
        }
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_resouce: &'a CameraResource,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera_resouce.bind_group(), &[]);
        render_pass.set_bind_group(1, self.texture_resource.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
