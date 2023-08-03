use super::Kind;
use glam::*;

#[derive(Debug, Clone)]
pub struct IUnit {
    pub position: IVec3,
    pub kind: Kind,
}

impl IUnit {
    pub fn new(position: IVec3, kind: Kind) -> Self {
        Self { position, kind }
    }

    pub fn breakable(&self) -> bool {
        match self.kind {
            Kind::SurfaceDirt
            | Kind::SurfaceGrass
            | Kind::SurfaceGravel
            | Kind::SurfaceSand
            | Kind::SurfaceStone => false,
            _ => true,
        }
    }
}
