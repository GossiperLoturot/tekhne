use glam::*;
use uuid::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitKind {
    Player,
}

impl UnitKind {
    #[inline]
    pub fn breakable(&self) -> bool {
        match self {
            UnitKind::Player => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: Uuid,
    pub position: Vec3A,
    pub kind: UnitKind,
}

impl Unit {
    #[inline]
    pub fn new(id: Uuid, position: Vec3A, kind: UnitKind) -> Self {
        Self { id, position, kind }
    }

    #[inline]
    pub fn breakable(&self) -> bool {
        self.kind.breakable()
    }
}
