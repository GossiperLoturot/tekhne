use crate::service;

mod camera;
mod unit;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    camera_resource: camera::CameraResource,
    unit_pipeline: unit::UnitPipeline,
}

impl Renderer {
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

        let camera_pipeline = camera::CameraResource::new(&device, &config);
        let unit_pipeline = unit::UnitPipeline::new(&device, &config, &camera_pipeline);

        Self {
            device,
            queue,
            surface,
            camera_resource: camera_pipeline,
            unit_pipeline,
        }
    }

    pub fn draw(&mut self, service: &service::Service) {
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
                ops: wgpu::Operations::default(),
            })],
            depth_stencil_attachment: None,
        });

        self.camera_resource.pre_draw(&self.queue, &service);
        self.unit_pipeline.pre_draw(&self.queue, &service);

        self.unit_pipeline
            .draw(&mut render_pass, &self.camera_resource);

        drop(render_pass);
        self.queue.submit([encoder.finish()]);
        frame.present();
    }
}
