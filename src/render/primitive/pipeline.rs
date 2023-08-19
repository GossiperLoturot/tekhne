//! プリミティブの描写に関するモジュール

use super::{
    model::{ModelItem, TextureItem},
    texture::AtlasResource,
};
use crate::{
    render::{camera::CameraResource, depth::DepthResource},
    system::System,
};
use glam::*;
use std::num::NonZeroU64;

/// プリミティブの描写に使用する頂点データ
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texcoord: [f32; 2],
}

impl Vertex {
    const ATTRIBUTES: &[wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    /// 頂点データのレイアウト
    fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRIBUTES,
        }
    }
}

/// 1つのアトラスマップに関連する頂点データとインデクスデータ
struct PageBatch {
    cache_vertices: Vec<Vertex>,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    cache_indices: Vec<u32>,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

/// プリミティブの描写を行うパイプライン
///
/// テクスチャはアトラスマップにまとめられる。複数枚のアトラスマップが生成された場合は
/// 複数の描写(バッチ)処理を行う。
pub struct Pipeline {
    page_batches: Vec<PageBatch>,
    texture_resource: AtlasResource,
    pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    /// 新しいパイプラインを作成する。
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        camera_resource: &CameraResource,
    ) -> Self {
        // アトラスマップを生成する。
        let texture_resource = AtlasResource::new(device, queue);

        // それぞれのアトラスマップに必要なデータ[`PageBatch`]を確保する
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
                cache_vertices: vec![],
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

        let shader = device.create_shader_module(wgpu::include_wgsl!(
            "../../../assets/shaders/primitive.wgsl"
        ));

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
            page_batches,
            texture_resource,
            pipeline,
        }
    }

    /// ブロックとエンティティをもとに、GPU上のデータを更新する。
    ///
    /// # Panic
    ///
    /// 存在しないアトラスマップ・バッチを使用しようとした場合
    pub fn pre_draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        staging_belt: &mut wgpu::util::StagingBelt,
        service: &System,
    ) {
        if let Some(camera) = service.camera.get_camera() {
            let bounds = camera.view_bounds();

            // 描写範囲内の[`crate::model::Block`]を描写用データへ変換する
            let blocks = service
                .block
                .get_by_aabb(bounds.floor().as_iaabb3())
                .into_iter()
                .map(|(_, block)| {
                    let position = block.position.as_vec3();
                    let model = ModelItem::from(block.kind);
                    let texture = TextureItem::from(block.kind);
                    (position, model, texture)
                });

            // 描写範囲内の[`crate::model::Entity`]を描写用データへ変換する
            let entities = service
                .entity
                .get_by_aabb(bounds)
                .into_iter()
                .map(|(_, entity)| {
                    let position = entity.position.into();
                    let model = ModelItem::from(entity.kind);
                    let texture = TextureItem::from(entity.kind);
                    (position, model, texture)
                });

            // 描写範囲内のすべての[`crate::model::Block`]と[`crate::model::Entity`]を該当するバッチへ
            // 頂点データとインデクスデータを挿入する。
            for (position, model, texture) in Iterator::chain(blocks, entities) {
                let texcoord = self
                    .texture_resource
                    .texcoord(&texture)
                    .expect("not found available texcoord");

                let page_batch = self
                    .page_batches
                    .get_mut(texcoord.page as usize)
                    .expect("not found available page batch");

                // メモリ確保が頻繁に行われるのを回避するためキャッシュを使用する。
                let vertices = &mut page_batch.cache_vertices;
                let indices = &mut page_batch.cache_indices;

                for index in model.indices() {
                    let vertex_count = vertices.len() as u32;
                    indices.push(vertex_count + index);
                }

                for vertex in model.vertices() {
                    let position = [
                        position.x + vertex.position[0],
                        position.y + vertex.position[1],
                        position.z + vertex.position[2],
                    ];
                    let texcoord = [
                        texcoord.x + vertex.texcoord[0] * texcoord.width,
                        texcoord.y + vertex.texcoord[1] * texcoord.height,
                    ];
                    vertices.push(Vertex { position, texcoord });
                }
            }

            // それぞれのバッチを実行する。
            for page in 0..self.texture_resource.page_count() {
                let page_batch = self
                    .page_batches
                    .get_mut(page as usize)
                    .expect("not found available page batch");

                let vertex_data = bytemuck::cast_slice(&page_batch.cache_vertices);
                page_batch.vertex_count = page_batch.cache_vertices.len() as u32;
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

                page_batch.cache_vertices.clear();
                page_batch.cache_indices.clear();
            }
        }
    }

    /// レンダーパスへ描写命令を発行する。
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
