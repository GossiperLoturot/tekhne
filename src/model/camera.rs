use super::{aabb3a, Aabb3A};
use glam::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3A,
    pub zoom: f32,
}

impl Camera {
    #[inline]
    pub fn new(position: Vec3A, zoom: f32) -> Self {
        Self { position, zoom }
    }

    #[inline]
    pub fn view_matrix(&self) -> Mat4 {
        let view_aabb = self.view_aabb();
        Mat4::orthographic_rh(
            view_aabb.min.x,
            view_aabb.max.x,
            view_aabb.min.y,
            view_aabb.max.y,
            view_aabb.min.z,
            view_aabb.max.z,
        ) * mat4(
            vec4(1.0, 0.0, 0.0, 0.0),
            vec4(0.0, 1.0, 0.0, 0.0),
            vec4(0.0, 1.0, 1.0, 0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline]
    pub fn view_aabb(&self) -> Aabb3A {
        aabb3a(
            self.position - Vec3A::splat(self.zoom),
            self.position + Vec3A::splat(self.zoom),
        )
    }
}
