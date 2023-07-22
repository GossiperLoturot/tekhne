use glam::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl IUnitKind {
    pub fn entry() -> [IUnitKind; 10] {
        [
            Self::SurfaceDirt,
            Self::SurfaceGrass,
            Self::SurfaceGravel,
            Self::SurfaceSand,
            Self::SurfaceStone,
            Self::MixGrass,
            Self::Dandelion,
            Self::FallenBranch,
            Self::FallenLeaves,
            Self::MixPebbles,
        ]
    }

    pub fn top_texture(&self) -> Option<image::DynamicImage> {
        let bytes: Option<&[u8]> = match self {
            Self::SurfaceDirt => Some(include_bytes!("../../assets/textures/surface_dirt.png")),
            Self::SurfaceGrass => Some(include_bytes!("../../assets/textures/surface_grass.png")),
            Self::SurfaceGravel => Some(include_bytes!("../../assets/textures/surface_gravel.png")),
            Self::SurfaceSand => Some(include_bytes!("../../assets/textures/surface_sand.png")),
            Self::SurfaceStone => Some(include_bytes!("../../assets/textures/surface_stone.png")),
            Self::MixGrass => None,
            Self::Dandelion => None,
            Self::FallenBranch => None,
            Self::FallenLeaves => None,
            Self::MixPebbles => None,
        };

        bytes.and_then(|bytes| image::load_from_memory(bytes).ok())
    }

    pub fn side_texture(&self) -> Option<image::DynamicImage> {
        let bytes: Option<&[u8]> = match self {
            Self::SurfaceDirt => None,
            Self::SurfaceGrass => None,
            Self::SurfaceGravel => None,
            Self::SurfaceSand => None,
            Self::SurfaceStone => None,
            Self::MixGrass => Some(include_bytes!("../../assets/textures/mix_grass.png")),
            Self::Dandelion => Some(include_bytes!("../../assets/textures/dandelion.png")),
            Self::FallenBranch => Some(include_bytes!("../../assets/textures/fallen_branch.png")),
            Self::FallenLeaves => Some(include_bytes!("../../assets/textures/fallen_leaves.png")),
            Self::MixPebbles => Some(include_bytes!("../../assets/textures/mix_pebbles.png")),
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
            _ => true,
        }
    }
}
