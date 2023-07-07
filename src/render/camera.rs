use crate::service;
use glam::*;

pub struct CameraResource {
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    width: u32,
    height: u32,
    screen_to_world: Option<Mat4>,
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

        Self {
            buffer,
            bind_group_layout,
            bind_group,
            width: config.width,
            height: config.height,
            screen_to_world: None,
        }
    }

    pub fn pre_draw(&mut self, queue: &wgpu::Queue, service: &service::Service) {
        self.screen_to_world = None;

        if let Some(camera) = service.camera.get_camera() {
            let view_area = camera.view_area();
            let matrix = Mat4::from_scale(Vec3::new(
                (self.height as f32 / self.width as f32).max(1.0),
                (self.width as f32 / self.height as f32).max(1.0),
                1.0,
            )) * Mat4::orthographic_rh(
                view_area.min.x as f32,
                view_area.max.x as f32,
                view_area.min.y as f32,
                view_area.max.y as f32,
                view_area.min.z as f32,
                view_area.max.z as f32,
            );

            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[matrix]));

            let screen_to_view = Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
                * Mat4::from_scale(Vec3::new(
                    (self.width as f32).recip() * 2.0,
                    -(self.height as f32).recip() * 2.0,
                    1.0,
                ));
            self.screen_to_world = Some(matrix.inverse() * screen_to_view);
        }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn screen_to_world(&self) -> Option<Mat4> {
        self.screen_to_world
    }
}
