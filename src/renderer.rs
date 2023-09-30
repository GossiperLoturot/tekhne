//! 描写に関するモジュール

use crate::game_loop;

pub mod camera;
pub mod depth;
pub mod entity;

/// 描写の機能
pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    staging_belt: wgpu::util::StagingBelt,
    camera_resource: camera::CameraResource,
    depth_resource: depth::DepthResource,
    entity_renderer: entity::EntityRenderer,
}

impl Renderer {
    /// 新しいコンテキストを作成する。(非同期)
    ///
    /// # Panic
    ///
    /// 互換性のある`Adapter`、`Surface`が存在しない場合
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
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        let depth_resource = depth::DepthResource::new(&device, &config);
        let camera_resource = camera::CameraResource::new(&device, &config);
        let entity_renderer = entity::EntityRenderer::new(&device, &config, &camera_resource);

        Self {
            device,
            queue,
            surface,
            staging_belt,
            camera_resource,
            depth_resource,
            entity_renderer,
        }
    }

    /// 描写サイクルを実行する。
    ///
    /// 描写の際に生成したデータをゲームサイクル内で使用するため、
    /// フィードバックデータ[`ReadBack`]を返す。
    ///
    /// # Panic
    ///
    /// 画面テクスチャの取得に失敗した場合
    pub fn draw(&mut self, game_loop: &game_loop::GameLoop) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        self.staging_belt.recall();

        self.camera_resource.upload(
            &self.device,
            &mut encoder,
            &mut self.staging_belt,
            game_loop,
        );
        self.entity_renderer.upload(
            &self.device,
            &mut encoder,
            &mut self.staging_belt,
            game_loop,
        );

        self.staging_belt.finish();

        let frame = self.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
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

        self.entity_renderer
            .draw(&mut render_pass, &self.camera_resource);

        drop(render_pass);
        self.queue.submit([encoder.finish()]);
        frame.present();
    }
}
