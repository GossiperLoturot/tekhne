use super::ResourceKind;
use glam::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: Uuid,
    pub position: Vec3A,
    pub resource_kind: ResourceKind,
}

impl Unit {
    pub fn new(id: Uuid, position: Vec3A, resource_kind: ResourceKind) -> Self {
        Self {
            id,
            position,
            resource_kind,
        }
    }
}
