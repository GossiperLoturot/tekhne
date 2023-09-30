//! エンティティの描写に関するモジュール

use std::num;

use crate::{
    game_loop,
    renderer::{camera, depth},
};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute] = &wgpu::vertex_attr_array![0 => Float32x3];

    fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRIBUTES,
        }
    }
}

pub struct EntityRenderer {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    pipeline: wgpu::RenderPipeline,
}

impl EntityRenderer {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera_resource: &camera::CameraResource,
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[camera_resource.bind_group_layout()],
            push_constant_ranges: &[],
        });

        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../../assets/shaders/entity.wgsl"));

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
                format: depth::DepthResource::DEPTH_FORMAT,
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
            index_count: 0,
            pipeline,
        }
    }

    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        staging_belt: &mut wgpu::util::StagingBelt,
        game_loop: &game_loop::GameLoop,
    ) {
        if let Some(camera) = game_loop.camera.get_camera() {
            let bounds = camera.view_bounds();

            let mut indices = vec![];
            let mut vertices = vec![];
            for (_, entity) in game_loop.entity.get_from_area(bounds) {
                let vlen = vertices.len() as u32;
                indices.push(vlen);
                indices.push(vlen + 1);
                indices.push(vlen + 2);
                indices.push(vlen + 2);
                indices.push(vlen + 3);
                indices.push(vlen);

                let bounds = entity.bounds();
                vertices.push(Vertex {
                    position: [bounds.min.x, bounds.min.y, 0.0],
                });
                vertices.push(Vertex {
                    position: [bounds.max.x, bounds.min.y, 0.0],
                });
                vertices.push(Vertex {
                    position: [bounds.max.x, bounds.max.y, 0.0],
                });
                vertices.push(Vertex {
                    position: [bounds.min.x, bounds.max.y, 0.0],
                });
            }

            let vertex_data = bytemuck::cast_slice(&vertices);
            if let Some(size) = num::NonZeroU64::new(vertex_data.len() as u64) {
                staging_belt
                    .write_buffer(encoder, &self.vertex_buffer, 0, size, device)
                    .copy_from_slice(vertex_data);
            }

            let index_data = bytemuck::cast_slice(&indices);
            if let Some(size) = num::NonZeroU64::new(index_data.len() as u64) {
                staging_belt
                    .write_buffer(encoder, &self.index_buffer, 0, size, device)
                    .copy_from_slice(index_data);
            }

            self.index_count = indices.len() as u32;
        }
    }

    pub fn draw<'a>(
        &'a mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_resource: &'a camera::CameraResource,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera_resource.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
