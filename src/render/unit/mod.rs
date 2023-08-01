use self::{texture::UnitTextureResource, unit_kind::UnitShape};
use super::{CameraResource, DepthResource};
use crate::service::Service;
use ahash::AHashMap;
use glam::*;
use strum::IntoEnumIterator;
use wgpu::util::DeviceExt;

mod texture;
mod unit_kind;

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

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    position_min: [f32; 3],
    position_max: [f32; 3],
    texcoord: [f32; 4],
}

impl Instance {
    const ATTRIBUTES: &[wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![2 => Float32x3, 3 => Float32x3, 4 => Float32x4];

    fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBUTES,
        }
    }
}

type ShapeGroupId = UnitShape;
struct ShapeGroup {
    id: ShapeGroupId,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

type InstanceGroupId = (UnitShape, u32);
struct InstanceGroup {
    id: InstanceGroupId,
    instance_buffer: wgpu::Buffer,
    instance_count: u32,
}

pub struct UnitPipeline {
    texture_resource: UnitTextureResource,
    shape_groups: AHashMap<ShapeGroupId, ShapeGroup>,
    instance_groups: AHashMap<InstanceGroupId, InstanceGroup>,
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

        let mut shape_groups = AHashMap::new();
        for id in UnitShape::iter() {
            #[rustfmt::skip]
            let vertices: &[Vertex] = match id {
                UnitShape::Block => &[
                    Vertex { position: [0.0, 0.0, 0.0], texcoord: [0.0, 1.0] },
                    Vertex { position: [1.0, 0.0, 0.0], texcoord: [1.0, 1.0] },
                    Vertex { position: [1.0, 1.0, 1.0], texcoord: [1.0, 0.0] },
                    Vertex { position: [0.0, 1.0, 1.0], texcoord: [0.0, 0.0] },
                ],
                UnitShape::Top => &[
                    Vertex { position: [0.0, 0.0, 1.0], texcoord: [0.0, 1.0] },
                    Vertex { position: [1.0, 0.0, 1.0], texcoord: [1.0, 1.0] },
                    Vertex { position: [1.0, 1.0, 1.0], texcoord: [1.0, 0.0] },
                    Vertex { position: [0.0, 1.0, 1.0], texcoord: [0.0, 0.0] },
                ],
                UnitShape::Bottom => &[
                    Vertex { position: [0.0, 0.0, 0.0], texcoord: [0.0, 1.0] },
                    Vertex { position: [1.0, 0.0, 0.0], texcoord: [1.0, 1.0] },
                    Vertex { position: [1.0, 1.0, 0.0], texcoord: [1.0, 0.0] },
                    Vertex { position: [0.0, 1.0, 0.0], texcoord: [0.0, 0.0] },
                ],
                UnitShape::Quad => &[
                    Vertex { position: [0.0, 0.0, 0.0], texcoord: [0.0, 1.0] },
                    Vertex { position: [1.0, 0.0, 0.0], texcoord: [1.0, 1.0] },
                    Vertex { position: [1.0, 0.0, 1.0], texcoord: [1.0, 0.0] },
                    Vertex { position: [0.0, 0.0, 1.0], texcoord: [0.0, 0.0] },
                ],
            };

            let indices: &[u16] = &[0, 1, 2, 2, 3, 0];
            let index_count = indices.len() as u32;

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            shape_groups.insert(
                id,
                ShapeGroup {
                    id,
                    vertex_buffer,
                    index_buffer,
                    index_count,
                },
            );
        }

        let page_count = texture_resource.page_count();
        let mut instance_groups = AHashMap::new();
        for shape in UnitShape::iter() {
            for page in 0..page_count {
                let id = (shape, page);

                let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                    label: None,
                    size: device.limits().max_buffer_size,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

                instance_groups.insert(
                    id,
                    InstanceGroup {
                        id,
                        instance_buffer,
                        instance_count: 0,
                    },
                );
            }
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
            shape_groups,
            instance_groups,
            texture_resource,
            pipeline,
        }
    }

    pub fn pre_draw(&mut self, queue: &wgpu::Queue, service: &Service) {
        if let Some(camera) = service.camera.get_camera() {
            let view_aabb = camera.view_aabb();

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

            let mut instances = AHashMap::new();
            for (origin, unit_kind) in Iterator::chain(units, iunits) {
                let shape = unit_kind.shape();
                let shape_aabb = unit_kind.shape_size();
                let atlas_texcoord = self
                    .texture_resource
                    .texcoord(&unit_kind)
                    .unwrap_or_else(|| panic!("not registered unit kind {:?}", &unit_kind));

                let position_min = (origin + shape_aabb.min).into();
                let position_max = (origin + shape_aabb.max).into();
                let texcoord = [
                    atlas_texcoord.x,
                    atlas_texcoord.y,
                    atlas_texcoord.width,
                    atlas_texcoord.height,
                ];

                instances
                    .entry((shape, atlas_texcoord.page))
                    .or_insert(vec![])
                    .push(Instance {
                        position_min,
                        position_max,
                        texcoord,
                    });
            }

            for (id, instances) in instances {
                let instance_group = self
                    .instance_groups
                    .get_mut(&id)
                    .expect("failed to get instance group");

                instance_group.instance_count = instances.len() as u32;
                queue.write_buffer(
                    &instance_group.instance_buffer,
                    0,
                    bytemuck::cast_slice(&instances),
                );
            }
        }
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_resouce: &'a CameraResource,
    ) {
        for (shape, page) in self.instance_groups.keys().copied() {
            let shape_group = self
                .shape_groups
                .get(&shape)
                .expect("failed to get shape group");

            let instance_group = self
                .instance_groups
                .get(&(shape, page))
                .expect("failed to get instance group");

            let texture_resource = self
                .texture_resource
                .bind_group(page)
                .expect("failed to get texture resource");

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, camera_resouce.bind_group(), &[]);
            render_pass.set_bind_group(1, texture_resource, &[]);
            render_pass.set_vertex_buffer(0, shape_group.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_group.instance_buffer.slice(..));
            render_pass.set_index_buffer(
                shape_group.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(
                0..shape_group.index_count,
                0,
                0..instance_group.instance_count,
            );
        }
    }
}
