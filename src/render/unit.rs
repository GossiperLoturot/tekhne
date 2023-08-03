use super::{texture::UnitTextureResource, CameraResource, DepthResource};
use crate::{model::Shape, service::Service};
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

pub struct UnitPipeline {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    unit_texture: UnitTextureResource,
    pipeline: wgpu::RenderPipeline,
}

impl UnitPipeline {
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

        let texture_resource = UnitTextureResource::new(device, queue);

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
            unit_texture: texture_resource,
            pipeline,
        }
    }

    pub fn pre_draw(&mut self, queue: &wgpu::Queue, service: &Service) {
        if let Some(camera) = service.camera.get_camera() {
            let view_aabb = camera.view_aabb();

            let mut vertices = vec![];
            let mut indices = vec![];

            let units = service
                .unit
                .get_units(view_aabb)
                .into_iter()
                .map(|unit| (unit.position, unit.kind));

            let iunits = service
                .iunit
                .get_iunits(view_aabb.as_iaabb3())
                .into_iter()
                .map(|iunit| (iunit.position.as_vec3a(), iunit.kind));

            Iterator::chain(units, iunits).for_each(|(origin, kind)| {
                let shape_aabb = kind.shape_size();
                let texcoord_aabb = self
                    .unit_texture
                    .get_texcoord(&kind)
                    .unwrap_or_else(|| panic!("not registered kind {:?}", &kind));

                let vertex_count = vertices.len() as u32;

                match kind.shape() {
                    Shape::Block => {
                        let position = (origin
                            + Vec3A::new(shape_aabb.min.x, shape_aabb.min.y, shape_aabb.min.y))
                        .into();
                        let texcoord = Vec2::new(texcoord_aabb.min.x, texcoord_aabb.max.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position = (origin
                            + Vec3A::new(shape_aabb.max.x, shape_aabb.min.y, shape_aabb.min.y))
                        .into();
                        let texcoord = Vec2::new(texcoord_aabb.max.x, texcoord_aabb.max.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position = (origin
                            + Vec3A::new(shape_aabb.max.x, shape_aabb.max.y, shape_aabb.max.y))
                        .into();
                        let texcoord = Vec2::new(texcoord_aabb.max.x, texcoord_aabb.min.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position = (origin
                            + Vec3A::new(shape_aabb.min.x, shape_aabb.max.y, shape_aabb.max.y))
                        .into();
                        let texcoord = Vec2::new(texcoord_aabb.min.x, texcoord_aabb.min.y).into();
                        vertices.push(Vertex { position, texcoord });
                    }
                    Shape::Top => {
                        let position =
                            (origin + Vec3A::new(shape_aabb.min.x, shape_aabb.min.y, 0.5)).into();
                        let texcoord = Vec2::new(texcoord_aabb.min.x, texcoord_aabb.max.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position =
                            (origin + Vec3A::new(shape_aabb.max.x, shape_aabb.min.y, 0.5)).into();
                        let texcoord = Vec2::new(texcoord_aabb.max.x, texcoord_aabb.max.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position =
                            (origin + Vec3A::new(shape_aabb.max.x, shape_aabb.max.y, 0.5)).into();
                        let texcoord = Vec2::new(texcoord_aabb.max.x, texcoord_aabb.min.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position =
                            (origin + Vec3A::new(shape_aabb.min.x, shape_aabb.max.y, 0.5)).into();
                        let texcoord = Vec2::new(texcoord_aabb.min.x, texcoord_aabb.min.y).into();
                        vertices.push(Vertex { position, texcoord });
                    }
                    Shape::Bottom => {
                        let position =
                            (origin + Vec3A::new(shape_aabb.min.x, shape_aabb.min.y, -0.49)).into();
                        let texcoord = Vec2::new(texcoord_aabb.min.x, texcoord_aabb.max.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position =
                            (origin + Vec3A::new(shape_aabb.max.x, shape_aabb.min.y, -0.49)).into();
                        let texcoord = Vec2::new(texcoord_aabb.max.x, texcoord_aabb.max.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position =
                            (origin + Vec3A::new(shape_aabb.max.x, shape_aabb.max.y, -0.49)).into();
                        let texcoord = Vec2::new(texcoord_aabb.max.x, texcoord_aabb.min.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position =
                            (origin + Vec3A::new(shape_aabb.min.x, shape_aabb.max.y, -0.49)).into();
                        let texcoord = Vec2::new(texcoord_aabb.min.x, texcoord_aabb.min.y).into();
                        vertices.push(Vertex { position, texcoord });
                    }
                    Shape::Quad => {
                        let position =
                            (origin + Vec3A::new(shape_aabb.min.x, 0.0, shape_aabb.min.y)).into();
                        let texcoord = Vec2::new(texcoord_aabb.min.x, texcoord_aabb.max.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position =
                            (origin + Vec3A::new(shape_aabb.max.x, 0.0, shape_aabb.min.y)).into();
                        let texcoord = Vec2::new(texcoord_aabb.max.x, texcoord_aabb.max.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position =
                            (origin + Vec3A::new(shape_aabb.max.x, 0.0, shape_aabb.max.y)).into();
                        let texcoord = Vec2::new(texcoord_aabb.max.x, texcoord_aabb.min.y).into();
                        vertices.push(Vertex { position, texcoord });

                        let position =
                            (origin + Vec3A::new(shape_aabb.min.x, 0.0, shape_aabb.max.y)).into();
                        let texcoord = Vec2::new(texcoord_aabb.min.x, texcoord_aabb.min.y).into();
                        vertices.push(Vertex { position, texcoord });
                    }
                }

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
        render_pass.set_bind_group(1, self.unit_texture.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
