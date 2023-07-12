use glam::*;

#[derive(Debug, Clone)]
pub struct Player {
    pub position: Vec3A,
}

impl Player {
    pub fn new(position: Vec3A) -> Self {
        Self { position }
    }
}
