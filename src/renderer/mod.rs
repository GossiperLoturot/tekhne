use crate::service;

mod iunit;
mod player;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    player_resource: player::PlayerResource,
    iunit_pipeline: iunit::IUnitPipeline,
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

        let player_pipeline = player::PlayerResource::new(&device);
        let iunit_pipeline = iunit::IUnitPipeline::new(&device, &config, &player_pipeline);

        Self {
            device,
            queue,
            surface,
            player_resource: player_pipeline,
            iunit_pipeline,
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

        self.player_resource
            .pre_draw(&self.queue, &service.player_service);
        self.iunit_pipeline
            .pre_draw(&self.queue, &service.iunit_service, &service.player_service);

        self.iunit_pipeline
            .draw(&mut render_pass, &self.player_resource);

        drop(render_pass);
        self.queue.submit([encoder.finish()]);
        frame.present();
    }
}
