use glam::*;

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
}

#[derive(Debug, Clone)]
pub struct IUnit {
    pub position: IVec3,
    pub kind: IUnitKind,
}

impl IUnit {
    pub fn new(position: IVec3, kind: IUnitKind) -> Self {
        Self { position, kind }
    }
}
