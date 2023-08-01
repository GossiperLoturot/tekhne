use super::Kind;
use glam::*;
use uuid::*;

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: Uuid,
    pub position: Vec3A,
    pub kind: Kind,
}

impl Unit {
    pub fn new(id: Uuid, position: Vec3A, kind: Kind) -> Self {
        Self { id, position, kind }
    }

    pub fn breakable(&self) -> bool {
        match self.kind {
            Kind::Player => false,
            _ => true,
        }
    }
}
