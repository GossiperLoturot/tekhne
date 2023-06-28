use crate::model::{Bounds, Camera};
use glam::*;

#[derive(Debug, Default)]
pub struct CameraService {
    camera: Option<Camera>,
}

impl CameraService {
    pub fn spawn_camera(&mut self) {
        if self.camera.is_some() {
            panic!("camera already exists");
        }

        const VIEW_SIZE: f32 = 32.0;
        self.camera = Some(Camera::new(
            Vec3A::ZERO,
            Bounds::new(Vec3A::splat(-VIEW_SIZE), Vec3A::splat(VIEW_SIZE)),
            VIEW_SIZE,
        ));
    }

    pub fn get_camera(&self) -> Option<&Camera> {
        self.camera.as_ref()
    }
}
