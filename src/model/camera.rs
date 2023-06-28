use super::Bounds;
use glam::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec3A,
    pub view_area: Bounds<Vec3A>,
    pub zoom: f32,
}

impl Camera {
    pub fn new(position: Vec3A, view_area: Bounds<Vec3A>, zoom: f32) -> Self {
        Self {
            position,
            view_area,
            zoom,
        }
    }
}
