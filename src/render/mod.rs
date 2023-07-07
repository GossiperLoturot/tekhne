use crate::service;

mod camera;
mod texture;
mod unit;

pub struct Render {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    camera_resource: camera::CameraResource,
    texture_resource: texture::TextureResource,
    unit_pipeline: unit::UnitPipeline,
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

        let camera_resource = camera::CameraResource::new(&device, &config);
        let texture_resource = texture::TextureResource::new(&device, &queue);
        let unit_pipeline =
            unit::UnitPipeline::new(&device, &config, &camera_resource, &texture_resource);

        Self {
            device,
            queue,
            surface,
            camera_resource,
            texture_resource,
            unit_pipeline,
        }
    }

    pub fn draw(&mut self, service: &service::Service) -> service::ReadBack {
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
        self.unit_pipeline
            .pre_draw(&self.queue, &service, &self.texture_resource);

        self.unit_pipeline.draw(
            &mut render_pass,
            &self.camera_resource,
            &self.texture_resource,
        );

        drop(render_pass);
        self.queue.submit([encoder.finish()]);
        frame.present();

        let screen_to_world = self.camera_resource.screen_to_world();

        service::ReadBack { screen_to_world }
    }
}
