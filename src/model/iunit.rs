use super::ResourceKind;
use glam::*;

#[derive(Debug, Clone)]
pub struct IUnit {
    pub position: IVec3,
    pub resource_kind: ResourceKind,
}

impl IUnit {
    pub fn new(position: IVec3, resource_kind: ResourceKind) -> Self {
        Self {
            position,
            resource_kind,
        }
    }
}
