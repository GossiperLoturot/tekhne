use super::Bounds;
use glam::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3A,
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: Vec3A, zoom: f32) -> Self {
        Self { position, zoom }
    }

    pub fn view_area(&self) -> Bounds<Vec3A> {
        Bounds::new(
            self.position - Vec3A::splat(self.zoom),
            self.position + Vec3A::splat(self.zoom),
        )
    }
}
