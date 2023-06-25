use glam::*;

#[derive(Debug, Clone)]
pub struct IUnit {
    pub pos: IVec3,
    pub resource_name: String,
}

impl IUnit {
    pub fn new(pos: IVec3, resource_name: String) -> Self {
        Self { pos, resource_name }
    }
}
