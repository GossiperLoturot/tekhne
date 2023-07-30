use super::Aabb3A;
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

    pub fn view_matrix(&self) -> Mat4 {
        let view_aabb = self.view_aabb();

        Mat4::orthographic_rh(
            view_aabb.min.x,
            view_aabb.max.x,
            view_aabb.min.y,
            view_aabb.max.y,
            view_aabb.min.z,
            view_aabb.max.z,
        ) * Mat4::from_cols(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn view_aabb(&self) -> Aabb3A {
        Aabb3A::new(
            self.position - Vec3A::splat(self.zoom),
            self.position + Vec3A::splat(self.zoom),
        )
    }
}
