//! 描写に関するモジュール

use crate::{assets, game_loop};

mod base;
mod block;
mod camera;
mod entity;
mod gui;

pub struct RenderState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface,
    pub staging_belt: wgpu::util::StagingBelt,
}

impl RenderState {
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

        Self {
            device,
            queue,
            config,
            surface,
            staging_belt,
        }
    }

    pub fn resize(&mut self, new_inner_size: winit::dpi::PhysicalSize<u32>) {
        let winit::dpi::PhysicalSize { width, height } = new_inner_size;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }
}

/// 描写の機能
pub struct Renderer {
    render_state: RenderState,
    camera_resource: camera::CameraResource,
    base_renderer: base::BaseRenderer,
    block_renderer: block::BlockRenderer,
    entity_renderer: entity::EntityRenderer,
    gui_renderer: gui::GuiRenderer,
}

impl Renderer {
    /// 新しいコンテキストを作成する。(非同期)
    ///
    /// # Panic
    ///
    /// 互換性のある`Adapter`、`Surface`が存在しない場合
    pub async fn new_async(assets: &assets::Assets, window: &winit::window::Window) -> Self {
        let render_state = RenderState::new_async(window).await;
        let camera_resource = camera::CameraResource::new(&render_state);

        let base_renderer = base::BaseRenderer::new(&render_state, assets, &camera_resource);
        let block_renderer = block::BlockRenderer::new(&render_state, assets, &camera_resource);
        let entity_renderer = entity::EntityRenderer::new(&render_state, assets, &camera_resource);

        let gui_renderer = gui::GuiRenderer::new(&render_state);

        Self {
            render_state,
            camera_resource,
            base_renderer,
            block_renderer,
            entity_renderer,
            gui_renderer,
        }
    }

    pub fn resize(&mut self, new_inner_size: winit::dpi::PhysicalSize<u32>) {
        self.render_state.resize(new_inner_size);
        self.camera_resource.resize(&self.render_state);
    }

    /// 描写サイクルを実行する。
    ///
    /// 描写の際に生成したデータをゲームサイクル内で使用するため、
    /// フィードバックデータ[`ReadBack`]を返す。
    ///
    /// # Panic
    ///
    /// 画面テクスチャの取得に失敗した場合
    pub fn render(
        &mut self,
        assets: &assets::Assets,
        extract: &game_loop::Extract,
        gui_cx: &egui::Context,
        gui_output: egui::FullOutput,
    ) {
        let mut encoder = self
            .render_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        self.camera_resource
            .upload(&mut self.render_state, &mut encoder, extract);
        self.base_renderer
            .upload(&mut self.render_state, &mut encoder, assets, extract);
        self.block_renderer
            .upload(&mut self.render_state, &mut encoder, assets, extract);
        self.entity_renderer
            .upload(&mut self.render_state, &mut encoder, assets, extract);

        let gui_resource_handle =
            self.gui_renderer
                .upload(&mut self.render_state, &mut encoder, gui_cx, gui_output);

        let frame = self.render_state.surface.get_current_texture().unwrap();
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
        self.gui_renderer
            .render(&mut render_pass, &gui_resource_handle);
        drop(render_pass);

        self.render_state.staging_belt.finish();
        self.render_state.queue.submit([encoder.finish()]);
        self.render_state.staging_belt.recall();
        frame.present();
    }
}
