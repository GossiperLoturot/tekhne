use super::ResourceKind;
use glam::*;

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: String,
    pub position: Vec3A,
    pub resource_kind: ResourceKind,
}

impl Unit {
    pub fn new(id: String, position: Vec3A, resource_kind: ResourceKind) -> Self {
        Self {
            id,
            position,
            resource_kind,
        }
    }
}
