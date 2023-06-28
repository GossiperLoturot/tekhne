use crate::service;
use glam::*;

pub struct CameraResource {
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    aspect_ratio: f32,
}

impl CameraResource {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
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
                resource: buffer.as_entire_binding(),
            }],
        });

        let aspect_ratio = config.width as f32 / config.height as f32;

        Self {
            buffer,
            bind_group_layout,
            bind_group,
            aspect_ratio,
        }
    }

    pub fn pre_draw(&self, queue: &wgpu::Queue, service: &service::Service) {
        if let Some(camera) = service.camera_service.get_camera() {
            let matrix = Mat4::orthographic_lh(
                camera.view_area.min.x as f32 * self.aspect_ratio,
                camera.view_area.max.x as f32 * self.aspect_ratio,
                camera.view_area.min.y as f32,
                camera.view_area.max.y as f32,
                camera.view_area.min.z as f32,
                camera.view_area.max.z as f32,
            );

            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[matrix]));
        }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
