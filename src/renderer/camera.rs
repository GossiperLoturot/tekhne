//! カメラに関するモジュール

use std::num;

use glam::*;

use crate::{game_loop, renderer};

/// デプスマップに使用するテクスチャのフォーマット[`wgpu::TextureFormat`]
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// カメラの変換行列と画面のアスペクト比の計算と保持を行うリソース
pub struct CameraResource {
    matrix_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    depth_view: wgpu::TextureView,
}

impl CameraResource {
    /// 新しいリソースを作成する。
    pub fn new(rendering_state: &renderer::RenderingState) -> Self {
        let device = &rendering_state.device;
        let config = &rendering_state.config;

        let matrix_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: std::mem::size_of::<Mat4>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: matrix_buffer.as_entire_binding(),
            }],
        });

        let depth_texture = create_depth_texture(device, config.width, config.height);
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            matrix_buffer,
            bind_group_layout,
            bind_group,
            depth_view,
        }
    }

    pub fn resize(&mut self, render_state: &renderer::RenderingState) {
        let device = &render_state.device;
        let config = &render_state.config;
        let depth_texture = create_depth_texture(device, config.width, config.height);
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.depth_view = depth_view;
    }

    /// 変換行列の計算とGPU上の行列データの更新を行う。
    pub fn upload(
        &mut self,
        rendering_state: &mut renderer::RenderingState,
        encoder: &mut wgpu::CommandEncoder,
        extract: &game_loop::Extract,
    ) {
        let device = &rendering_state.device;
        let staging_belt = &mut rendering_state.staging_belt;

        let matrix = extract.matrix;

        if let Some(size) = num::NonZeroU64::new(self.matrix_buffer.size()) {
            staging_belt
                .write_buffer(encoder, &self.matrix_buffer, 0, size, device)
                .copy_from_slice(bytemuck::cast_slice(&[matrix]));
        }
    }

    pub fn render_pass<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        frame_view: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: frame_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        })
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    })
}
