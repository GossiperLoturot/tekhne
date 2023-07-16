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
        let aabb = self.view_aabb();

        Mat4::orthographic_rh(
            aabb.min.x, aabb.max.x, aabb.min.y, aabb.max.y, aabb.min.z, aabb.max.z,
        ) * Mat4 {
            x_axis: Vec4::new(1.0, 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, 1.0, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 1.0, 1.0, 0.0),
            w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn view_aabb(&self) -> Aabb3A {
        Aabb3A::new(
            self.position - Vec3A::splat(self.zoom),
            self.position + Vec3A::splat(self.zoom),
        )
    }
}
