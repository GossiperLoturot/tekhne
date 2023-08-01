use super::UnitKind;
use glam::*;

#[derive(Debug, Clone)]
pub struct IUnit {
    pub position: IVec3,
    pub kind: UnitKind,
}

impl IUnit {
    pub fn new(position: IVec3, kind: UnitKind) -> Self {
        Self { position, kind }
    }

    pub fn breakable(&self) -> bool {
        match self.kind {
            UnitKind::SurfaceDirt
            | UnitKind::SurfaceGrass
            | UnitKind::SurfaceGravel
            | UnitKind::SurfaceSand
            | UnitKind::SurfaceStone => false,
            _ => true,
        }
    }
}
