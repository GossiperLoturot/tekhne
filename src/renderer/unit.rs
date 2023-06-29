use super::{camera, texture};
use crate::service;
use glam::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    position: Vec3,
    texcoord: Vec2,
}

impl Instance {
    const ATTRIBUTES: &[wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBUTES,
        }
    }
}

#[derive(Debug)]
pub struct UnitPipeline {
    instance_buffer: wgpu::Buffer,
    instance_count: u32,
    pipeline: wgpu::RenderPipeline,
}

impl UnitPipeline {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_resource: &camera::CameraResource,
        texture_resource: &texture::TextureResource,
    ) -> Self {
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
                buffers: &[Instance::layout()],
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
            depth_stencil: None,
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
            instance_buffer,
            instance_count: 0,
            pipeline,
        }
    }

    pub fn pre_draw(
        &mut self,
        queue: &wgpu::Queue,
        service: &service::Service,
        texture_resource: &texture::TextureResource,
    ) {
        let iunits = service
            .iunit_service
            .get_iunits(
                service
                    .camera_service
                    .get_camera()
                    .map(|camera| camera.view_area.into())
                    .unwrap_or_default(),
            )
            .into_iter()
            .map(|iunit| Instance {
                position: iunit.pos.as_vec3(),
                texcoord: texture_resource.texcoord(iunit.resource_kind).as_vec2(),
            });

        let units = service
            .unit_service
            .get_units(
                service
                    .camera_service
                    .get_camera()
                    .map(|camera| camera.view_area)
                    .unwrap_or_default(),
            )
            .into_iter()
            .map(|unit| Instance {
                position: unit.pos.into(),
                texcoord: texture_resource.texcoord(unit.resource_kind).as_vec2(),
            });

        let instance_data = iunits.chain(units).collect::<Vec<_>>();

        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&instance_data),
        );
        self.instance_count = instance_data.len() as u32;
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_resouce: &'a camera::CameraResource,
        texture_resource: &'a texture::TextureResource,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera_resouce.bind_group(), &[]);
        render_pass.set_bind_group(1, texture_resource.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        render_pass.draw(0..6, 0..self.instance_count);
    }
}
