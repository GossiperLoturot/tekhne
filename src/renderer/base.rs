//! ベースの描写に関するモジュール

use std::num;

use aabb::*;
use glam::*;
use wgpu::util::DeviceExt;

use crate::{
    assets,
    game_loop::{self, base},
    renderer::{camera, depth},
};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pub position: [f32; 3],
    pub texcoord: [f32; 2],
}

impl Vertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute] =
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

struct Batch {
    vertices: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    indices: Vec<u32>,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    bind_group: wgpu::BindGroup,
}

impl Batch {
    #[inline]
    fn new(
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        bind_group: wgpu::BindGroup,
    ) -> Self {
        Self {
            vertices: Vec::new(),
            vertex_buffer,
            vertex_count: 0,
            indices: Vec::new(),
            index_buffer,
            index_count: 0,
            bind_group,
        }
    }
}

pub struct BaseRenderer {
    texcoords: Vec<image_atlas::Texcoord32>,
    batches: Vec<Batch>,
    pipeline: wgpu::RenderPipeline,
}

impl BaseRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        assets: &assets::Assets,
        camera_resource: &camera::CameraResource,
    ) -> Self {
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

        const ATLAS_MAX_COUNT: u32 = 8;
        const ATLAS_SIZE: u32 = 1024;
        const ATLAS_BLOCK_SIZE: u32 = 32;
        const ATLAS_MIP_FILTER: image_atlas::AtlasMipFilter = image_atlas::AtlasMipFilter::Lanczos3;
        let atlas = image_atlas::create_atlas(&image_atlas::AtlasDescriptor {
            max_page_count: ATLAS_MAX_COUNT,
            size: ATLAS_SIZE,
            mip: image_atlas::AtlasMipOption::MipWithBlock(ATLAS_MIP_FILTER, ATLAS_BLOCK_SIZE),
            entries: &assets
                .base_specs
                .iter()
                .map(|spec| image_atlas::AtlasEntry {
                    texture: image::open(&spec.texture_path).unwrap(),
                    mip: spec.texture_mip_option,
                })
                .collect::<Vec<_>>(),
        })
        .unwrap();

        let texcoords = atlas
            .texcoords
            .into_iter()
            .map(|texcoord| texcoord.to_f32())
            .collect::<Vec<_>>();

        let batches = atlas
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
                    &texture
                        .mip_maps
                        .into_iter()
                        .flat_map(|texture| texture.to_vec())
                        .collect::<Vec<_>>(),
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

                Batch::new(vertex_buffer, index_buffer, bind_group)
            })
            .collect::<Vec<_>>();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[camera_resource.bind_group_layout(), &bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("base.wgsl"));
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
            texcoords,
            batches,
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

            game_loop
                .base
                .get_by_bounds(base::Bounds::View(bounds))
                .for_each(|(_, base)| {
                    let bounds = iaabb2(base.position, base.position + IVec2::ONE).as_aabb2();
                    let texcoord = &self.texcoords[base.spec_id];
                    let batch = &mut self.batches[texcoord.page as usize];

                    let vertex_count = batch.vertices.len() as u32;
                    batch.indices.push(vertex_count);
                    batch.indices.push(vertex_count + 1);
                    batch.indices.push(vertex_count + 2);
                    batch.indices.push(vertex_count + 2);
                    batch.indices.push(vertex_count + 3);
                    batch.indices.push(vertex_count);

                    const BASE_Z: f32 = -0.00390625; // z = -2^(-8)
                    batch.vertices.push(Vertex {
                        position: [bounds.min.x, bounds.min.y, BASE_Z],
                        texcoord: [texcoord.min_x, texcoord.max_y],
                    });
                    batch.vertices.push(Vertex {
                        position: [bounds.max.x, bounds.min.y, BASE_Z],
                        texcoord: [texcoord.max_x, texcoord.max_y],
                    });
                    batch.vertices.push(Vertex {
                        position: [bounds.max.x, bounds.max.y, BASE_Z],
                        texcoord: [texcoord.max_x, texcoord.min_y],
                    });
                    batch.vertices.push(Vertex {
                        position: [bounds.min.x, bounds.max.y, BASE_Z],
                        texcoord: [texcoord.min_x, texcoord.min_y],
                    });
                });

            for batch in &mut self.batches {
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
    }

    pub fn draw<'a>(
        &'a mut self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_resource: &'a camera::CameraResource,
    ) {
        for batch in &self.batches {
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, camera_resource.bind_group(), &[]);
            render_pass.set_bind_group(1, &batch.bind_group, &[]);
            render_pass.set_vertex_buffer(0, batch.vertex_buffer.slice(..));
            render_pass.set_index_buffer(batch.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..batch.index_count, 0, 0..1);
        }
    }
}
