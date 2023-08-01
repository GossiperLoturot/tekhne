pub use camera::CameraResource;
pub use depth::DepthResource;
pub use ui::UIPipeline;
pub use unit::UnitPipeline;

use crate::service::{ReadBack, Service};

mod camera;
mod depth;
mod texture;
mod ui;
mod unit;

pub struct Render {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    camera_resource: CameraResource,
    depth_resource: DepthResource,
    unit_pipeline: UnitPipeline,
    ui_pipeline: UIPipeline,
}

impl Render {
    pub async fn new_async(window: &winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        let inner_size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, inner_size.width, inner_size.height)
            .unwrap();
        surface.configure(&device, &config);

        let camera_resource = CameraResource::new(&device, &config);
        let depth_resource = DepthResource::new(&device, &config);
        let unit_pipeline = UnitPipeline::new(&device, &queue, &config, &camera_resource);
        let ui_pipeline = UIPipeline::new(&device, &queue, &config);

        Self {
            device,
            queue,
            surface,
            camera_resource,
            depth_resource,
            unit_pipeline,
            ui_pipeline,
        }
    }

    pub fn draw(&mut self, service: &Service) -> ReadBack {
        self.camera_resource.pre_draw(&self.queue, service);
        self.unit_pipeline.pre_draw(&self.queue, service);
        self.ui_pipeline.pre_draw(&self.queue, service);

        let frame = self.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: self.depth_resource.view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        self.unit_pipeline
            .draw(&mut render_pass, &self.camera_resource);
        self.ui_pipeline.draw(&mut render_pass);

        drop(render_pass);
        self.queue.submit([encoder.finish()]);
        frame.present();

        let screen_to_world_matrix = self.camera_resource.screen_to_world_matrix();
        let screen_to_ui_matrix = self.ui_pipeline.screen_to_ui_matrix();

        ReadBack {
            screen_to_world_matrix,
            screen_to_ui_matrix,
        }
    }
}
