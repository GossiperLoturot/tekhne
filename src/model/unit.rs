use super::UnitKind;
use glam::*;
use uuid::*;

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: Uuid,
    pub position: Vec3A,
    pub kind: UnitKind,
}

impl Unit {
    pub fn new(id: Uuid, position: Vec3A, kind: UnitKind) -> Self {
        Self { id, position, kind }
    }

    pub fn breakable(&self) -> bool {
        match self.kind {
            UnitKind::Player => false,
            _ => true,
        }
    }
}
