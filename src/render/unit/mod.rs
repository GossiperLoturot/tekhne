use self::{
    model::{UnitModelItem, UnitTextureItem},
    texture::UnitTextureResource,
};
use super::{CameraResource, DepthResource};
use crate::service::Service;
use glam::*;
use std::num::NonZeroU64;

mod model;
mod texture;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UnitVertex {
    pub position: [f32; 3],
    pub texcoord: [f32; 2],
}

impl UnitVertex {
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

pub struct PageBatch {
    chache_vertices: Vec<UnitVertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    cache_indices: Vec<u32>,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

pub struct UnitPipeline {
    page_batches: Vec<PageBatch>,
    texture_resource: UnitTextureResource,
    pipeline: wgpu::RenderPipeline,
}

impl UnitPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        camera_resource: &CameraResource,
    ) -> Self {
        let texture_resource = UnitTextureResource::new(device, queue);

        let mut page_batches = vec![];
        for _ in 0..texture_resource.page_count() {
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

            let page_batch = PageBatch {
                chache_vertices: vec![],
                vertex_buffer,
                vertex_count: 0,
                cache_indices: vec![],
                index_buffer,
                index_count: 0,
            };
            page_batches.push(page_batch);
        }

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                camera_resource.bind_group_layout(),
                texture_resource.bind_group_layout(),
            ],
            push_constant_ranges: &[],
        });

        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../../../assets/shaders/unit.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[UnitVertex::layout()],
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
            page_batches,
            texture_resource,
            pipeline,
        }
    }

    pub fn pre_draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        staging_belt: &mut wgpu::util::StagingBelt,
        service: &Service,
    ) {
        if let Some(camera) = service.camera.get_camera() {
            let view_aabb = camera.view_aabb();

            let iunits = service
                .iunit
                .get_iunits(view_aabb.as_iaabb3())
                .into_iter()
                .map(|iunit| {
                    let position = iunit.position.as_vec3();
                    let model = UnitModelItem::from(iunit.kind);
                    let texture = UnitTextureItem::from(iunit.kind);
                    (position, model, texture)
                });

            let units = service.unit.get_units(view_aabb).into_iter().map(|unit| {
                let position = unit.position.into();
                let model = UnitModelItem::from(unit.kind);
                let texture = UnitTextureItem::from(unit.kind);
                (position, model, texture)
            });

            for (position, model, texture) in Iterator::chain(units, iunits) {
                let texcoord = self
                    .texture_resource
                    .texcoord(&texture)
                    .expect("not found available texcoord");

                let page_batch = self
                    .page_batches
                    .get_mut(texcoord.page as usize)
                    .expect("not found available page batch");

                let vertices = &mut page_batch.chache_vertices;
                let indices = &mut page_batch.cache_indices;

                for index in model.indices() {
                    let vertex_count = vertices.len();
                    indices.push(vertex_count as u32 + index);
                }

                for vertex in model.vertices() {
                    let vertex = UnitVertex {
                        position: [
                            position.x + vertex.position[0],
                            position.y + vertex.position[1],
                            position.z + vertex.position[2],
                        ],
                        texcoord: [
                            texcoord.x + vertex.texcoord[0] * texcoord.width,
                            texcoord.y + vertex.texcoord[1] * texcoord.height,
                        ],
                    };
                    vertices.push(vertex)
                }
            }

            for page in 0..self.texture_resource.page_count() {
                let page_batch = self
                    .page_batches
                    .get_mut(page as usize)
                    .expect("not found available page batch");

                let vertex_data = bytemuck::cast_slice(&page_batch.chache_vertices);
                page_batch.vertex_count = page_batch.chache_vertices.len() as u32;
                if let Some(size) = NonZeroU64::new(vertex_data.len() as u64) {
                    staging_belt
                        .write_buffer(encoder, &page_batch.vertex_buffer, 0, size, device)
                        .copy_from_slice(vertex_data);
                }

                let index_data = bytemuck::cast_slice(&page_batch.cache_indices);
                page_batch.index_count = page_batch.cache_indices.len() as u32;
                if let Some(size) = NonZeroU64::new(index_data.len() as u64) {
                    staging_belt
                        .write_buffer(encoder, &page_batch.index_buffer, 0, size, device)
                        .copy_from_slice(index_data);
                }

                page_batch.chache_vertices.clear();
                page_batch.cache_indices.clear();
            }
        }
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_resouce: &'a CameraResource,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, camera_resouce.bind_group(), &[]);

        for page in 0..self.texture_resource.page_count() {
            let texture_bind_group = self
                .texture_resource
                .bind_group(page)
                .expect("not found available atlas page");

            let page_batch = &self
                .page_batches
                .get(page as usize)
                .expect("not found available page batch");

            let vertex_buffer = &page_batch.vertex_buffer;
            let index_buffer = &page_batch.index_buffer;
            let index_count = page_batch.index_count;

            render_pass.set_bind_group(1, texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..index_count, 0, 0..1);
        }
    }
}
