//! カメラに関するモジュール

use std::num;

use glam::*;

use crate::{game_loop, renderer};

pub struct ReadBack {
    pub screen_to_world: Mat4,
}

/// カメラの変換行列と画面のアスペクト比の計算と保持を行うリソース
pub struct CameraResource {
    matrix_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    width: u32,
    height: u32,
}

impl CameraResource {
    /// 新しいリソースを作成する。
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
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

        Self {
            matrix_buffer,
            bind_group_layout,
            bind_group,
            width: config.width,
            height: config.height,
        }
    }

    /// 変換行列の計算とGPU上の行列データの更新を行う。
    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        staging_belt: &mut wgpu::util::StagingBelt,
        cx: &renderer::InputContext,
        extract: &game_loop::GameExtract,
    ) -> ReadBack {
        // アスペクト比による引き延ばしを補正する行列
        // 描写空間の中に画面が収まるように補正する。
        let correction_matrix = Mat4::from_scale(vec3(
            (self.height as f32 / self.width as f32).max(1.0),
            (self.width as f32 / self.height as f32).max(1.0),
            1.0,
        ));
        let matrix = correction_matrix * extract.view_matrix;

        if let Some(size) = num::NonZeroU64::new(self.matrix_buffer.size()) {
            staging_belt
                .write_buffer(encoder, &self.matrix_buffer, 0, size, device)
                .copy_from_slice(bytemuck::cast_slice(&[matrix]));
        }

        // ピクセル座標空間から[0,1]座標空間へ変換する行列
        let transform_matrix = Mat4::from_translation(vec3(-1.0, 1.0, 0.0))
            * Mat4::from_scale(vec3(
                (self.width as f32).recip() * 2.0,
                -(self.height as f32).recip() * 2.0,
                1.0,
            ));

        let screen_to_world = matrix.inverse() * transform_matrix;
        ReadBack { screen_to_world }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
