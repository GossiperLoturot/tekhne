pub use camera::UICameraResource;
pub use inventory::UIInventoryPipeline;

use crate::service::Service;
use glam::*;

mod camera;
mod inventory;

pub struct UIPipeline {
    camera_resource: UICameraResource,
    inventory: UIInventoryPipeline,
}

impl UIPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let camera_resource = UICameraResource::new(device, config);
        let inventory = UIInventoryPipeline::new(device, queue, config, &camera_resource);

        Self {
            camera_resource,
            inventory,
        }
    }

    pub fn pre_draw(&mut self, queue: &wgpu::Queue, service: &Service) {
        self.camera_resource.pre_draw(queue);
        self.inventory.pre_draw(queue, service);
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.inventory.draw(render_pass, &self.camera_resource);
    }

    pub fn screen_to_ui_matrix(&self) -> Option<Mat4> {
        self.camera_resource.screen_to_ui_matrix()
    }
}
