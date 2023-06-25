use glam::*;

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: String,
    pub pos: Vec3A,
    pub resource_name: String,
}

impl Unit {
    pub fn new(id: String, pos: Vec3A, resource_name: String) -> Self {
        Self {
            id,
            pos,
            resource_name,
        }
    }
}
