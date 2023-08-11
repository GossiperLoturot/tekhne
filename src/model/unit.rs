use super::{aabb3a, Aabb3A};
use glam::*;

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

    #[inline]
    pub fn aabb(&self) -> Aabb3A {
        match self {
            UnitKind::Player => aabb3a(vec3a(-0.5, -0.5, 0.0), vec3a(0.5, 0.5, 2.0)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub position: Vec3A,
    pub kind: UnitKind,
}

impl Unit {
    #[inline]
    pub fn new(position: Vec3A, kind: UnitKind) -> Self {
        Self { position, kind }
    }

    #[inline]
    pub fn breakable(&self) -> bool {
        self.kind.breakable()
    }

    #[inline]
    pub fn aabb(&self) -> Aabb3A {
        self.position + self.kind.aabb()
    }
}
