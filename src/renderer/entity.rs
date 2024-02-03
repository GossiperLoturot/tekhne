//! エンティティの描写に関するモジュール

use std::num;

use aabb::*;
use glam::*;
use wgpu::util::DeviceExt;

use crate::{
    assets, game_loop,
    renderer::{self, camera},
};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pub position: [f32; 3],
    pub texcoord: [f32; 2],
}

impl Vertex {
    const ATTRIBUTES: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    #[inline]
    fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRIBUTES,
        }
    }
}

struct BatchBuffer {
    vertices: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    indices: Vec<u32>,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    bind_group: wgpu::BindGroup,
}

pub struct EntityRenderer {
    texcoord_handles: Vec<image_atlas::Texcoord32>,
    batch_buffers: Vec<BatchBuffer>,
    pipeline: wgpu::RenderPipeline,
}

impl EntityRenderer {
    pub fn new(
        render_state: &renderer::RenderState,
        assets: &assets::Assets,
        camera_resource: &camera::CameraResource,
    ) -> Self {
        let device = &render_state.device;
        let queue = &render_state.queue;
        let config = &render_state.config;

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let entries = assets
            .entity_specs
            .iter()
            .map(|spec| {
                let texture = image::open(&spec.texture_path).unwrap();
                let mip = spec.texture_mip_option;
                image_atlas::AtlasEntry { texture, mip }
            })
            .collect::<Vec<_>>();

        let atlas = image_atlas::create_atlas(&image_atlas::AtlasDescriptor {
            max_page_count: 8,
            size: 1024,
            mip: image_atlas::AtlasMipOption::MipWithBlock(
                image_atlas::AtlasMipFilter::Lanczos3,
                32,
            ),
            entries: &entries,
        })
        .unwrap();

        let texcoord_handles = atlas
            .texcoords
            .into_iter()
            .map(|texcoord| texcoord.to_f32())
            .collect::<Vec<_>>();

        let batch_buffers = atlas
            .textures
            .into_iter()
            .map(|texture| {
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

                let texture_data = texture
                    .mip_maps
                    .into_iter()
                    .flat_map(|texture| texture.to_vec())
                    .collect::<Vec<_>>();
                let texture = device.create_texture_with_data(
                    queue,
                    &wgpu::TextureDescriptor {
                        label: None,
                        size: wgpu::Extent3d {
                            width: texture.size,
                            height: texture.size,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: texture.mip_level_count,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        usage: wgpu::TextureUsages::TEXTURE_BINDING,
                        view_formats: &[],
                    },
                    wgpu::util::TextureDataOrder::default(),
                    &texture_data,
                );
                let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                });

                BatchBuffer {
                    vertices: vec![],
                    vertex_buffer,
                    vertex_count: 0,
                    indices: vec![],
                    index_buffer,
                    index_count: 0,
                    bind_group,
                }
            })
            .collect::<Vec<_>>();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout, camera_resource.bind_group_layout()],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("entity.wgsl"));
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
                format: camera::DEPTH_FORMAT,
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
            texcoord_handles,
            batch_buffers,
            pipeline,
        }
    }

    pub fn upload(
        &mut self,
        render_state: &mut renderer::RenderState,
        encoder: &mut wgpu::CommandEncoder,
        assets: &assets::Assets,
        extract: &game_loop::Extract,
    ) {
        let device = &render_state.device;
        let staging_belt = &mut render_state.staging_belt;

        extract.entities.iter().for_each(|entity| {
            let spec = &assets.entity_specs[entity.spec_id];

            let bounds = aabb2(entity.position, entity.position) + spec.view_size;
            let texcoord = &self.texcoord_handles[entity.spec_id];
            let batch = &mut self.batch_buffers[texcoord.page as usize];

            let vertex_count = batch.vertices.len() as u32;
            batch.indices.push(vertex_count);
            batch.indices.push(vertex_count + 1);
            batch.indices.push(vertex_count + 2);
            batch.indices.push(vertex_count + 2);
            batch.indices.push(vertex_count + 3);
            batch.indices.push(vertex_count);

            let (negative_y2z, positive_y2z) = match spec.y_axis {
                assets::YAxis::Y => (0.0, 0.0),
                assets::YAxis::YZ => (spec.view_size.min.y, spec.view_size.max.y),
            };
            batch.vertices.push(Vertex {
                position: [bounds.min.x, bounds.min.y, negative_y2z],
                texcoord: [texcoord.min_x, texcoord.max_y],
            });
            batch.vertices.push(Vertex {
                position: [bounds.max.x, bounds.min.y, negative_y2z],
                texcoord: [texcoord.max_x, texcoord.max_y],
            });
            batch.vertices.push(Vertex {
                position: [bounds.max.x, bounds.max.y, positive_y2z],
                texcoord: [texcoord.max_x, texcoord.min_y],
            });
            batch.vertices.push(Vertex {
                position: [bounds.min.x, bounds.max.y, positive_y2z],
                texcoord: [texcoord.min_x, texcoord.min_y],
            });
        });

        for batch in &mut self.batch_buffers {
            let vertex_data = bytemuck::cast_slice(&batch.vertices);
            if let Some(size) = num::NonZeroU64::new(vertex_data.len() as u64) {
                staging_belt
                    .write_buffer(encoder, &batch.vertex_buffer, 0, size, device)
                    .copy_from_slice(vertex_data);
            }
            batch.vertex_count = batch.vertices.len() as u32;
            batch.vertices.clear();

            let index_data = bytemuck::cast_slice(&batch.indices);
            if let Some(size) = num::NonZeroU64::new(index_data.len() as u64) {
                staging_belt
                    .write_buffer(encoder, &batch.index_buffer, 0, size, device)
                    .copy_from_slice(index_data);
            }
            batch.index_count = batch.indices.len() as u32;
            batch.indices.clear();
        }
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_resource: &'a camera::CameraResource,
    ) {
        for batch in &self.batch_buffers {
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &batch.bind_group, &[]);
            render_pass.set_bind_group(1, camera_resource.bind_group(), &[]);
            render_pass.set_vertex_buffer(0, batch.vertex_buffer.slice(..));
            render_pass.set_index_buffer(batch.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..batch.index_count, 0, 0..1);
        }
    }
}
