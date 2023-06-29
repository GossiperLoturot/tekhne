use glam::*;

use super::ResourceKind;

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: String,
    pub pos: Vec3A,
    pub resource_kind: ResourceKind,
}

impl Unit {
    pub fn new(id: String, pos: Vec3A, resource_kind: ResourceKind) -> Self {
        Self {
            id,
            pos,
            resource_kind,
        }
    }
}
