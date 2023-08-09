use super::{iaabb3, IAabb3};
use glam::*;
use std::sync::atomic;

static COUNTER: atomic::AtomicU64 = atomic::AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IUnitKind {
    SurfaceDirt,
    SurfaceGrass,
    SurfaceGravel,
    SurfaceSand,
    SurfaceStone,
    MixGrass,
    Dandelion,
    FallenBranch,
    FallenLeaves,
    MixPebbles,
    OakTree,
    BirchTree,
    DyingTree,
    FallenTree,
    MixRock,
}

impl IUnitKind {
    #[inline]
    pub fn breakable(&self) -> bool {
        match self {
            Self::SurfaceDirt => false,
            Self::SurfaceGrass => false,
            Self::SurfaceGravel => false,
            Self::SurfaceSand => false,
            Self::SurfaceStone => false,
            Self::MixGrass => true,
            Self::Dandelion => true,
            Self::FallenBranch => true,
            Self::FallenLeaves => true,
            Self::MixPebbles => true,
            Self::OakTree => true,
            Self::BirchTree => true,
            Self::DyingTree => true,
            Self::FallenTree => true,
            Self::MixRock => true,
        }
    }

    #[inline]
    pub fn aabb(&self) -> IAabb3 {
        match self {
            Self::SurfaceDirt => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::SurfaceGrass => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::SurfaceGravel => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::SurfaceSand => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::SurfaceStone => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::MixGrass => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::Dandelion => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::FallenBranch => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::FallenLeaves => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::MixPebbles => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 0)),
            Self::OakTree => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 2)),
            Self::BirchTree => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 2)),
            Self::DyingTree => iaabb3(ivec3(0, 0, 0), ivec3(0, 0, 2)),
            Self::FallenTree => iaabb3(ivec3(0, 0, 0), ivec3(3, 1, 0)),
            Self::MixRock => iaabb3(ivec3(0, 0, 0), ivec3(1, 1, 0)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IUnit {
    pub id: u64,
    pub position: IVec3,
    pub kind: IUnitKind,
}

impl IUnit {
    #[inline]
    pub fn new(id: u64, position: IVec3, kind: IUnitKind) -> Self {
        Self { id, position, kind }
    }

    #[inline]
    pub fn create(position: IVec3, kind: IUnitKind) -> Self {
        let id = COUNTER.load(atomic::Ordering::SeqCst);
        COUNTER.fetch_add(1, atomic::Ordering::SeqCst);
        Self { id, position, kind }
    }

    #[inline]
    pub fn breakable(&self) -> bool {
        self.kind.breakable()
    }

    #[inline]
    pub fn aabb(&self) -> IAabb3 {
        self.position + self.kind.aabb()
    }
}
