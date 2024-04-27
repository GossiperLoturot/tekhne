//! 描写に関するモジュール

use crate::{assets, game_loop};

mod base;
mod block;
mod camera;
mod entity;

pub struct RenderingState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub staging_belt: wgpu::util::StagingBelt,
}

impl RenderingState {
    pub async fn new_async(window: std::rc::Rc<winit::window::Window>) -> Self {
        let inner_size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = unsafe {
            let target = wgpu::SurfaceTargetUnsafe::from_window(&*window).unwrap();
            instance.create_surface_unsafe(target).unwrap()
        };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        let config = surface
            .get_default_config(&adapter, inner_size.width, inner_size.height)
            .unwrap();
        surface.configure(&device, &config);
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Self {
            device,
            queue,
            config,
            surface,
            staging_belt,
        }
    }

    pub fn resize(&mut self, new_inner_size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = new_inner_size.width;
        self.config.height = new_inner_size.height;
        self.surface.configure(&self.device, &self.config);
    }
}

/// 描写の機能
pub struct RenderingSystem {
    rendering_state: RenderingState,
    camera_resource: camera::CameraResource,
    base_renderer: base::BaseRenderer,
    block_renderer: block::BlockRenderer,
    entity_renderer: entity::EntityRenderer,
}

impl RenderingSystem {
    /// 新しいコンテキストを作成する。(非同期)
    ///
    /// # Panic
    ///
    /// 互換性のある`Adapter`、`Surface`が存在しない場合
    pub async fn new_async(
        assets: std::rc::Rc<assets::Assets>,
        window: std::rc::Rc<winit::window::Window>,
    ) -> Self {
        let rendering_state = RenderingState::new_async(window).await;
        let camera_resource = camera::CameraResource::new(&rendering_state);
        let base_renderer =
            base::BaseRenderer::new(assets.clone(), &rendering_state, &camera_resource);
        let block_renderer =
            block::BlockRenderer::new(assets.clone(), &rendering_state, &camera_resource);
        let entity_renderer =
            entity::EntityRenderer::new(assets.clone(), &rendering_state, &camera_resource);

        Self {
            rendering_state,
            camera_resource,
            base_renderer,
            block_renderer,
            entity_renderer,
        }
    }

    pub fn resize(&mut self, new_inner_size: winit::dpi::PhysicalSize<u32>) {
        self.rendering_state.resize(new_inner_size);
        self.camera_resource.resize(&self.rendering_state);
    }

    /// 描写サイクルを実行する。
    ///
    /// # Panic
    ///
    /// 画面テクスチャの取得に失敗した場合
    pub fn draw(&mut self, extract: &game_loop::Extract) {
        let mut encoder = self
            .rendering_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        self.camera_resource
            .upload(&mut self.rendering_state, &mut encoder, extract);
        self.base_renderer
            .upload(&mut self.rendering_state, &mut encoder, extract);
        self.block_renderer
            .upload(&mut self.rendering_state, &mut encoder, extract);
        self.entity_renderer
            .upload(&mut self.rendering_state, &mut encoder, extract);

        let frame = self.rendering_state.surface.get_current_texture().unwrap();
        let frame_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut render_pass = self.camera_resource.render_pass(&mut encoder, &frame_view);
        self.base_renderer
            .render(&mut render_pass, &self.camera_resource);
        self.block_renderer
            .render(&mut render_pass, &self.camera_resource);
        self.entity_renderer
            .render(&mut render_pass, &self.camera_resource);
        drop(render_pass);

        self.rendering_state.staging_belt.finish();
        self.rendering_state.queue.submit([encoder.finish()]);
        self.rendering_state.staging_belt.recall();

        frame.present();
    }
}
