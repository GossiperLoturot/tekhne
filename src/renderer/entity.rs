//! エンティティの描写に関するモジュール

use std::num;

use aabb::*;
use ahash::HashMap;
use glam::*;
use wgpu::util::DeviceExt;

use crate::{
    game_loop::{self, entity::EntityKind},
    renderer::{camera, depth},
};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texcoord: [f32; 2],
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum TextureHandle {
    Player,
    SurfaceDirt,
    SurfaceGrass,
    SurfaceGravel,
    SurfaceSand,
    SurfaceStone,
    MixGrass,
    Dandelion,
    FallenBranch,
    FallenLeaves,
    MixPebbles,
    OakTree,
    BirchTree,
    DyingTree,
    FallenTree,
    MixRock,
}

impl TextureHandle {
    pub fn texture(&self) -> image::ImageResult<image::DynamicImage> {
        let bytes: &[u8] = match self {
            Self::Player => include_bytes!("../../assets/textures/frame.png"),
            Self::SurfaceDirt => include_bytes!("../../assets/textures/surface_dirt.png"),
            Self::SurfaceGrass => include_bytes!("../../assets/textures/surface_grass.png"),
            Self::SurfaceGravel => include_bytes!("../../assets/textures/surface_gravel.png"),
            Self::SurfaceSand => include_bytes!("../../assets/textures/surface_sand.png"),
            Self::SurfaceStone => include_bytes!("../../assets/textures/surface_stone.png"),
            Self::MixGrass => include_bytes!("../../assets/textures/mix_grass.png"),
            Self::Dandelion => include_bytes!("../../assets/textures/dandelion.png"),
            Self::FallenBranch => include_bytes!("../../assets/textures/fallen_branch.png"),
            Self::FallenLeaves => include_bytes!("../../assets/textures/fallen_leaves.png"),
            Self::MixPebbles => include_bytes!("../../assets/textures/mix_pebbles.png"),
            Self::OakTree => include_bytes!("../../assets/textures/oak_tree.png"),
            Self::BirchTree => include_bytes!("../../assets/textures/birch_tree.png"),
            Self::DyingTree => include_bytes!("../../assets/textures/dying_tree.png"),
            Self::FallenTree => include_bytes!("../../assets/textures/fallen_tree.png"),
            Self::MixRock => include_bytes!("../../assets/textures/mix_rock.png"),
        };

        image::load_from_memory(bytes)
    }
}

impl From<EntityKind> for TextureHandle {
    #[inline]
    fn from(value: EntityKind) -> Self {
        match value {
            EntityKind::Player => Self::Player,
            EntityKind::SurfaceDirt => Self::SurfaceDirt,
            EntityKind::SurfaceGrass => Self::SurfaceGrass,
            EntityKind::SurfaceGravel => Self::SurfaceGravel,
            EntityKind::SurfaceSand => Self::SurfaceSand,
            EntityKind::SurfaceStone => Self::SurfaceStone,
            EntityKind::MixGrass => Self::MixGrass,
            EntityKind::Dandelion => Self::Dandelion,
            EntityKind::FallenBranch => Self::FallenBranch,
            EntityKind::FallenLeaves => Self::FallenLeaves,
            EntityKind::MixPebbles => Self::MixPebbles,
            EntityKind::OakTree => Self::OakTree,
            EntityKind::BirchTree => Self::BirchTree,
            EntityKind::DyingTree => Self::DyingTree,
            EntityKind::FallenTree => Self::FallenTree,
            EntityKind::MixRock => Self::MixRock,
        }
    }
}

struct TextureResource {
    page_count: u32,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_groups: Vec<wgpu::BindGroup>,
    texcoords: HashMap<TextureHandle, (u32, Aabb2)>,
}

impl TextureResource {
    const ATLAS_PAGE_COUNT: u32 = 255;
    const ATLAS_SIZE: u32 = 2048;
    const ATLAS_BLOCK_SIZE: u32 = 32;

    fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let texture_handles = [
            TextureHandle::Player,
            TextureHandle::SurfaceDirt,
            TextureHandle::SurfaceGrass,
            TextureHandle::SurfaceGravel,
            TextureHandle::SurfaceSand,
            TextureHandle::SurfaceStone,
            TextureHandle::MixGrass,
            TextureHandle::Dandelion,
            TextureHandle::FallenBranch,
            TextureHandle::FallenLeaves,
            TextureHandle::MixPebbles,
            TextureHandle::OakTree,
            TextureHandle::BirchTree,
            TextureHandle::DyingTree,
            TextureHandle::FallenTree,
            TextureHandle::MixRock,
        ];

        let entries = texture_handles
            .into_iter()
            .map(|handle| atlas::AtlasEntry {
                key: handle,
                texture: handle.texture().unwrap(),
                leak: atlas::AtlasEntryLeakOption::Single,
            })
            .collect::<Vec<_>>();

        let result = atlas::create_atlas::<_, _, ahash::RandomState>(&atlas::AtlasDescriptor {
            page_count: Self::ATLAS_PAGE_COUNT,
            size: Self::ATLAS_SIZE,
            leak: atlas::AtlasLeakOption::Block(Self::ATLAS_BLOCK_SIZE),
            entries: &entries,
        })
        .unwrap();

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

        let page_count = result.atlas_textures.len() as u32;

        let texcoords = result
            .texcoords
            .into_iter()
            .map(|(handle, texcoord)| {
                let aabb = aabb2(
                    vec2(texcoord.norm_min_x(), texcoord.norm_min_y()),
                    vec2(texcoord.norm_max_x(), texcoord.norm_max_y()),
                );
                (handle, (texcoord.page, aabb))
            })
            .collect::<HashMap<_, _>>();

        let mut bind_groups = vec![];
        for atlas_texture in result.atlas_textures {
            let mip_level_count = atlas_texture.textures.len() as u32;

            let data = atlas_texture
                .textures
                .into_iter()
                .flat_map(|texture| texture.to_vec())
                .collect::<Vec<_>>();

            let texture = device.create_texture_with_data(
                queue,
                &wgpu::TextureDescriptor {
                    label: None,
                    size: wgpu::Extent3d {
                        width: Self::ATLAS_SIZE,
                        height: Self::ATLAS_SIZE,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                },
                &data,
            );
            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });

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
                        resource: wgpu::BindingResource::Sampler(&texture_sampler),
                    },
                ],
            });
            bind_groups.push(bind_group);
        }

        Self {
            page_count,
            bind_groups,
            bind_group_layout,
            texcoords,
        }
    }

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    fn bind_group(&self, page: u32) -> Option<&wgpu::BindGroup> {
        self.bind_groups.get(page as usize)
    }

    fn get_page_count(&self) -> u32 {
        self.page_count
    }

    fn get_texcoord(&self, texture: TextureHandle) -> Option<&(u32, Aabb2)> {
        self.texcoords.get(&texture)
    }
}

pub struct Batch {
    vertices: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    indices: Vec<u32>,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

pub struct EntityRenderer {
    texture_resource: TextureResource,
    batches: Vec<Batch>,
    pipeline: wgpu::RenderPipeline,
}

impl EntityRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        camera_resource: &camera::CameraResource,
    ) -> Self {
        let texture_resource = TextureResource::new(device, queue);

        let batch_count = texture_resource.get_page_count();
        let batches = (0..batch_count)
            .map(|_| {
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

                Batch {
                    vertices: vec![],
                    vertex_buffer,
                    vertex_count: 0,
                    indices: vec![],
                    index_buffer,
                    index_count: 0,
                }
            })
            .collect::<Vec<_>>();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                camera_resource.bind_group_layout(),
                texture_resource.bind_group_layout(),
            ],
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
            texture_resource,
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

            for (_, entity) in game_loop.entity.get_from_area(bounds) {
                let (page, texcoord) = *self
                    .texture_resource
                    .get_texcoord(entity.kind.into())
                    .unwrap();

                let batch = self.batches.get_mut(page as usize).unwrap();

                let vertex_count = batch.vertices.len() as u32;
                batch.indices.push(vertex_count);
                batch.indices.push(vertex_count + 1);
                batch.indices.push(vertex_count + 2);
                batch.indices.push(vertex_count + 2);
                batch.indices.push(vertex_count + 3);
                batch.indices.push(vertex_count);

                let bounds = entity.bounds();
                batch.vertices.push(Vertex {
                    position: [bounds.min.x, bounds.min.y, 0.0],
                    texcoord: [texcoord.min.x, texcoord.max.y],
                });
                batch.vertices.push(Vertex {
                    position: [bounds.max.x, bounds.min.y, 0.0],
                    texcoord: [texcoord.max.x, texcoord.max.y],
                });
                batch.vertices.push(Vertex {
                    position: [bounds.max.x, bounds.max.y, 0.0],
                    texcoord: [texcoord.max.x, texcoord.min.y],
                });
                batch.vertices.push(Vertex {
                    position: [bounds.min.x, bounds.max.y, 0.0],
                    texcoord: [texcoord.min.x, texcoord.min.y],
                });
            }

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
            let bind_group = self.texture_resource.bind_group(0).unwrap();

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, camera_resource.bind_group(), &[]);
            render_pass.set_bind_group(1, bind_group, &[]);
            render_pass.set_vertex_buffer(0, batch.vertex_buffer.slice(..));
            render_pass.set_index_buffer(batch.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..batch.index_count, 0, 0..1);
        }
    }
}
