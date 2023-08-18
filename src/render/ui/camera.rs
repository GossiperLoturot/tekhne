//! UIで使用されるカメラに関するモジュール

use glam::*;
use std::num::NonZeroU64;

/// 画面のアスペクト比の計算と保持を行うリソース
pub struct UICameraResource {
    matrix_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    width: u32,
    height: u32,
    screen_to_ui_matrix: Option<Mat4>,
}

impl UICameraResource {
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
            screen_to_ui_matrix: None,
        }
    }

    /// 変換行列の計算とGPU上の行列データの更新を行う。
    pub fn pre_draw(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        staging_belt: &mut wgpu::util::StagingBelt,
    ) {
        self.screen_to_ui_matrix = None;

        // アスペクト比による引き延ばしを補正する行列
        // 画面の中に描写空間が収まるように補正する。
        let correction_matrix = Mat4::from_scale(vec3(
            (self.height as f32 / self.width as f32).min(1.0),
            (self.width as f32 / self.height as f32).min(1.0),
            1.0,
        ));
        let matrix = correction_matrix;

        if let Some(size) = NonZeroU64::new(self.matrix_buffer.size()) {
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
        self.screen_to_ui_matrix = Some(matrix.inverse() * transform_matrix);
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    /// 画面座標空間からUI座標空間への変換行列を返す。
    pub fn screen_to_ui_matrix(&self) -> Option<Mat4> {
        self.screen_to_ui_matrix
    }
}
