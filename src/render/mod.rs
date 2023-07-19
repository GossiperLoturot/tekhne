pub use camera::CameraResource;
pub use depth::DepthResource;
pub use iunit::IUnitPipeline;
pub use iunit_texture::IUnitTextureResource;
pub use player::PlayerPipeline;
pub use ui::UIPipeline;
pub use ui_camera::UICameraResource;
pub use unit::UnitPipeline;
pub use unit_texture::UnitTextureResource;

use crate::service::{ReadBack, Service};

mod camera;
mod depth;
mod iunit;
mod iunit_texture;
mod player;
mod ui;
mod ui_camera;
mod unit;
mod unit_texture;

pub struct Render {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    camera_resource: CameraResource,
    iunit_texture_resource: IUnitTextureResource,
    unit_texture_resource: UnitTextureResource,
    depth_resource: DepthResource,
    iunit_pipeline: IUnitPipeline,
    unit_pipeline: UnitPipeline,
    player_pipeline: PlayerPipeline,
    ui_camera_resource: UICameraResource,
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
        let iunit_texture_resource = IUnitTextureResource::new(&device, &queue);
        let unit_texture_resource = UnitTextureResource::new(&device, &queue);
        let depth_resource = DepthResource::new(&device, &config);
        let iunit_pipeline =
            IUnitPipeline::new(&device, &config, &camera_resource, &iunit_texture_resource);
        let unit_pipeline =
            UnitPipeline::new(&device, &config, &camera_resource, &unit_texture_resource);
        let player_pipeline = PlayerPipeline::new(&device, &queue, &config, &camera_resource);
        let ui_camera_resource = UICameraResource::new(&device, &config);
        let ui_pipeline = UIPipeline::new(&device, &queue, &config, &ui_camera_resource);

        Self {
            device,
            queue,
            surface,
            camera_resource,
            iunit_texture_resource,
            unit_texture_resource,
            depth_resource,
            iunit_pipeline,
            unit_pipeline,
            player_pipeline,
            ui_camera_resource,
            ui_pipeline,
        }
    }

    pub fn draw(&mut self, service: &Service) -> ReadBack {
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

        self.camera_resource.pre_draw(&self.queue, service);
        self.iunit_pipeline
            .pre_draw(&self.queue, service, &self.iunit_texture_resource);
        self.unit_pipeline
            .pre_draw(&self.queue, service, &self.unit_texture_resource);
        self.player_pipeline.pre_draw(&self.queue, service);
        self.ui_camera_resource.pre_draw(&self.queue);
        self.ui_pipeline.pre_draw(&self.queue, service);

        self.iunit_pipeline.draw(
            &mut render_pass,
            &self.camera_resource,
            &self.iunit_texture_resource,
        );
        self.unit_pipeline.draw(
            &mut render_pass,
            &self.camera_resource,
            &self.unit_texture_resource,
        );
        self.player_pipeline
            .draw(&mut render_pass, &self.camera_resource);
        self.ui_pipeline
            .draw(&mut render_pass, &self.ui_camera_resource);

        drop(render_pass);
        self.queue.submit([encoder.finish()]);
        frame.present();

        let screen_to_world_matrix = self.camera_resource.screen_to_world_matrix();
        let screen_to_ui_matrix = self.ui_camera_resource.screen_to_ui_matrix();

        ReadBack {
            screen_to_world_matrix,
            screen_to_ui_matrix,
        }
    }
}
