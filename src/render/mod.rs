//! 描写に関するモジュール

use crate::system::{ReadBack, System};
use camera::CameraResource;
use depth::DepthResource;
use primitive::PrimitivePipeline;
use ui::UICameraResource;
use ui::UIInventoryPipeline;

mod camera;
mod depth;
mod primitive;
mod ui;

/// 描写に関する操作を行うコンテキスト
pub struct Render {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    staging_belt: wgpu::util::StagingBelt,
    camera_resource: CameraResource,
    depth_resource: DepthResource,
    primitive_pipeline: PrimitivePipeline,
    ui_camera_resource: UICameraResource,
    ui_inventoy_pipeline: UIInventoryPipeline,
}

impl Render {
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

        let camera_resource = CameraResource::new(&device, &config);
        let depth_resource = DepthResource::new(&device, &config);
        let primitive_pipeline = PrimitivePipeline::new(&device, &queue, &config, &camera_resource);
        let ui_camera_resource = UICameraResource::new(&device, &config);
        let ui_inventoy_pipeline =
            UIInventoryPipeline::new(&device, &queue, &config, &ui_camera_resource);

        Self {
            device,
            queue,
            surface,
            staging_belt,
            camera_resource,
            depth_resource,
            primitive_pipeline,
            ui_camera_resource,
            ui_inventoy_pipeline,
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
    pub fn draw(&mut self, service: &System) -> ReadBack {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        self.staging_belt.recall();

        self.camera_resource
            .pre_draw(&self.device, &mut encoder, &mut self.staging_belt, service);
        self.primitive_pipeline.pre_draw(
            &self.device,
            &mut encoder,
            &mut self.staging_belt,
            service,
        );
        self.ui_camera_resource
            .pre_draw(&self.device, &mut encoder, &mut self.staging_belt);

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

        self.primitive_pipeline
            .draw(&mut render_pass, &self.camera_resource);
        self.ui_inventoy_pipeline
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
