use super::ResourceKind;
use glam::*;

#[derive(Debug, Clone)]
pub struct IUnit {
    pub pos: IVec3,
    pub resource_kind: ResourceKind,
}

impl IUnit {
    pub fn new(pos: IVec3, resource_kind: ResourceKind) -> Self {
        Self { pos, resource_kind }
    }
}
