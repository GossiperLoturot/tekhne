use self::{
    extract::{UnitAtlasItem, UnitShapeItem},
    instance::{UnitInstance, UnitInstanceResource},
    shape::{UnitShapeResource, UnitVertex},
    texture::UnitAtlasResource,
};
use super::{CameraResource, DepthResource};
use crate::service::Service;
use strum::IntoEnumIterator;

mod extract;
mod instance;
mod shape;
mod texture;

pub struct UnitPipeline {
    atlas_resource: UnitAtlasResource,
    shape_resource: UnitShapeResource,
    instance_resource: UnitInstanceResource,
    pipeline: wgpu::RenderPipeline,
}

impl UnitPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        camera_resource: &CameraResource,
    ) -> Self {
        let atlas_resource =
            UnitAtlasResource::new(device, queue, &UnitAtlasItem::iter().collect::<Vec<_>>());
        let shape_resource =
            UnitShapeResource::new(device, &UnitShapeItem::iter().collect::<Vec<_>>());
        let instance_resource = UnitInstanceResource::new(device, &atlas_resource, &shape_resource);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                camera_resource.bind_group_layout(),
                atlas_resource.bind_group_layout(),
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
                buffers: &[UnitVertex::layout(), UnitInstance::layout()],
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
            atlas_resource,
            shape_resource,
            instance_resource,
            pipeline,
        }
    }

    pub fn pre_draw(&mut self, queue: &wgpu::Queue, service: &Service) {
        self.instance_resource
            .pre_draw(queue, service, &self.atlas_resource);
    }

    pub fn draw<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        camera_resouce: &'a CameraResource,
    ) {
        for page in 0..self.atlas_resource.page_count() {
            for shape in self.shape_resource.shapes() {
                let texture_resource = self
                    .atlas_resource
                    .bind_group(page)
                    .expect("failed to get texture resource");

                let shape_group = self
                    .shape_resource
                    .shape_group(shape)
                    .expect("failed to get shape group");

                let instance_group = self
                    .instance_resource
                    .instance_group(page, shape)
                    .expect("failed to get instance group");

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
}
