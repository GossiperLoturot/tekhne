use super::{CameraResource, DepthResource, TextureResource};
use crate::service::Service;
use glam::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: Vec3,
    texcoord: Vec2,
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

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    position: Vec3,
    texcoord: Vec2,
}

impl Instance {
    const ATTRIBUTES: &[wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![2 => Float32x3, 3 => Float32x2];

    fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBUTES,
        }
    }
}

#[derive(Debug)]
pub struct UnitPipeline {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    instance_count: u32,
    pipeline: wgpu::RenderPipeline,
}

impl UnitPipeline {
    #[rustfmt::skip]
    const VERTICES: [Vertex; 4] = [
        Vertex { position: Vec3::new(-0.5, -0.5, -0.5), texcoord: Vec2::new(0.0, 0.0) },
        Vertex { position: Vec3::new( 0.5, -0.5, -0.5), texcoord: Vec2::new(1.0, 0.0) },
        Vertex { position: Vec3::new( 0.5,  0.5, 0.5), texcoord: Vec2::new(1.0, 1.0) },
        Vertex { position: Vec3::new(-0.5,  0.5, 0.5), texcoord: Vec2::new(0.0, 1.0) },
    ];

    #[rustfmt::skip]
    const INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_resource: &CameraResource,
        texture_resource: &TextureResource,
    ) -> Self {
        use wgpu::util::DeviceExt;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&Self::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&Self::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: device.limits().max_buffer_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../../assets/shaders/unit.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                camera_resource.bind_group_layout(),
                texture_resource.bind_group_layout(),
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout(), Instance::layout()],
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
            index_buffer,
            instance_buffer,
            instance_count: 0,
            pipeline,
        }
    }

    pub fn pre_draw(
        &mut self,
        queue: &wgpu::Queue,
        service: &Service,
        texture_resource: &TextureResource,
    ) {
        if let Some(camera) = service.camera.get_camera() {
            let bounds = camera.view_bounds();

            let iunits = service
                .iunit
                .get_iunits(bounds.into())
                .into_iter()
                .map(|iunit| Instance {
                    position: iunit.position.as_vec3(),
                    texcoord: texture_resource.texcoord(iunit.resource_kind).as_vec2(),
                });

            let units = service
                .unit
                .get_units(bounds)
                .into_iter()
                .map(|unit| Instance {
                    position: unit.position.into(),
                    texcoord: texture_resource.texcoord(unit.resource_kind).as_vec2(),
                });

            let instances = iunits.chain(units).collect::<Vec<_>>();

            queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
            self.instance_count = instances.len() as u32;
        }
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_resouce: &'a CameraResource,
        texture_resource: &'a TextureResource,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera_resouce.bind_group(), &[]);
        render_pass.set_bind_group(1, texture_resource.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..Self::INDICES.len() as u32, 0, 0..self.instance_count);
    }
}
