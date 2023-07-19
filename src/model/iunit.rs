use glam::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IUnitKind {
    SurfaceDirt,
    SurfaceGrass,
    SurfaceGravel,
    SurfaceSand,
    SurfaceStone,
}

impl IUnitKind {
    pub fn entry() -> [IUnitKind; 5] {
        [
            IUnitKind::SurfaceDirt,
            IUnitKind::SurfaceGrass,
            IUnitKind::SurfaceGrass,
            IUnitKind::SurfaceSand,
            IUnitKind::SurfaceStone,
        ]
    }

    pub fn top_texture(&self) -> Option<image::DynamicImage> {
        let bytes: Option<&[u8]> = match self {
            IUnitKind::SurfaceDirt => {
                Some(include_bytes!("../../assets/textures/surface_dirt.png"))
            }
            IUnitKind::SurfaceGrass => {
                Some(include_bytes!("../../assets/textures/surface_grass.png"))
            }
            IUnitKind::SurfaceGravel => {
                Some(include_bytes!("../../assets/textures/surface_gravel.png"))
            }
            IUnitKind::SurfaceSand => {
                Some(include_bytes!("../../assets/textures/surface_sand.png"))
            }
            IUnitKind::SurfaceStone => {
                Some(include_bytes!("../../assets/textures/surface_stone.png"))
            }
        };

        bytes.and_then(|bytes| image::load_from_memory(bytes).ok())
    }

    pub fn side_texture(&self) -> Option<image::DynamicImage> {
        let bytes: Option<&[u8]> = match self {
            IUnitKind::SurfaceDirt => None,
            IUnitKind::SurfaceGrass => None,
            IUnitKind::SurfaceGravel => None,
            IUnitKind::SurfaceSand => None,
            IUnitKind::SurfaceStone => None,
        };

        bytes.and_then(|bytes| image::load_from_memory(bytes).ok())
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

    pub fn breakable(&self) -> bool {
        match self.kind {
            IUnitKind::SurfaceDirt => false,
            IUnitKind::SurfaceGrass => false,
            IUnitKind::SurfaceGravel => false,
            IUnitKind::SurfaceSand => false,
            IUnitKind::SurfaceStone => false,
        }
    }
}
