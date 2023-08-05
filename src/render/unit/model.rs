use super::{texture::UnitAtlasOption, UnitVertex};
use crate::model::*;
use strum::EnumIter;

pub enum UnitModelItem {
    TopPlane,
    BottomPlane,
    Billboard1x2,
    Billboard2x2,
    Billboard4x2,
    Billboard4x6,
}

impl UnitModelItem {
    #[rustfmt::skip]
    pub fn vertices(&self) -> &[UnitVertex] {
        match self {
            UnitModelItem::TopPlane => &[
                UnitVertex { position: [0.0, 0.0, 1.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [1.0, 0.0, 1.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [1.0, 1.0, 1.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [0.0, 1.0, 1.0], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::BottomPlane => &[
                UnitVertex { position: [0.0, 0.0, 0.1], texcoord: [0.0, 1.0] },
                UnitVertex { position: [1.0, 0.0, 0.1], texcoord: [1.0, 1.0] },
                UnitVertex { position: [1.0, 1.0, 0.1], texcoord: [1.0, 0.0] },
                UnitVertex { position: [0.0, 1.0, 0.1], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::Billboard1x2 => &[
                UnitVertex { position: [0.0, 0.0, 0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [1.0, 0.0, 0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [1.0, 0.0, 2.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [0.0, 0.0, 2.0], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::Billboard2x2 => &[
                UnitVertex { position: [0.0, 0.0, 0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [2.0, 0.0, 0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [2.0, 0.0, 2.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [0.0, 0.0, 2.0], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::Billboard4x2 => &[
                UnitVertex { position: [0.0, 0.0, 0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [4.0, 0.0, 0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [4.0, 0.0, 2.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [0.0, 0.0, 2.0], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::Billboard4x6 => &[
                UnitVertex { position: [0.0, 0.0, 0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [4.0, 0.0, 0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [4.0, 0.0, 6.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [0.0, 0.0, 6.0], texcoord: [0.0, 0.0] },
            ],
        }
    }

    pub fn indices(&self) -> &[u32] {
        match self {
            UnitModelItem::TopPlane => &[0, 1, 2, 2, 3, 0],
            UnitModelItem::BottomPlane => &[0, 1, 2, 2, 3, 0],
            UnitModelItem::Billboard1x2 => &[0, 1, 2, 2, 3, 0],
            UnitModelItem::Billboard2x2 => &[0, 1, 2, 2, 3, 0],
            UnitModelItem::Billboard4x2 => &[0, 1, 2, 2, 3, 0],
            UnitModelItem::Billboard4x6 => &[0, 1, 2, 2, 3, 0],
        }
    }
}

impl From<UnitKind> for UnitModelItem {
    fn from(value: UnitKind) -> Self {
        match value {
            UnitKind::SurfaceDirt => Self::TopPlane,
            UnitKind::SurfaceGrass => Self::TopPlane,
            UnitKind::SurfaceGravel => Self::TopPlane,
            UnitKind::SurfaceSand => Self::TopPlane,
            UnitKind::SurfaceStone => Self::TopPlane,
            UnitKind::MixGrass => Self::BottomPlane,
            UnitKind::Dandelion => Self::BottomPlane,
            UnitKind::FallenBranch => Self::BottomPlane,
            UnitKind::FallenLeaves => Self::BottomPlane,
            UnitKind::MixPebbles => Self::BottomPlane,
            UnitKind::Player => Self::Billboard1x2,
            UnitKind::OakTree => Self::Billboard4x6,
            UnitKind::BirchTree => Self::Billboard4x6,
            UnitKind::DyingTree => Self::Billboard4x6,
            UnitKind::FallenTree => Self::Billboard4x2,
            UnitKind::MixRock => Self::Billboard2x2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum UnitTextureItem {
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
    Player,
    OakTree,
    BirchTree,
    DyingTree,
    FallenTree,
    MixRock,
}

impl From<UnitKind> for UnitTextureItem {
    fn from(value: UnitKind) -> Self {
        match value {
            UnitKind::SurfaceDirt => Self::SurfaceDirt,
            UnitKind::SurfaceGrass => Self::SurfaceGrass,
            UnitKind::SurfaceGravel => Self::SurfaceGravel,
            UnitKind::SurfaceSand => Self::SurfaceSand,
            UnitKind::SurfaceStone => Self::SurfaceStone,
            UnitKind::MixGrass => Self::MixGrass,
            UnitKind::Dandelion => Self::Dandelion,
            UnitKind::FallenBranch => Self::FallenBranch,
            UnitKind::FallenLeaves => Self::FallenLeaves,
            UnitKind::MixPebbles => Self::MixPebbles,
            UnitKind::Player => Self::Player,
            UnitKind::OakTree => Self::OakTree,
            UnitKind::BirchTree => Self::BirchTree,
            UnitKind::DyingTree => Self::DyingTree,
            UnitKind::FallenTree => Self::FallenTree,
            UnitKind::MixRock => Self::MixRock,
        }
    }
}

impl UnitTextureItem {
    pub fn texture(&self) -> image::ImageResult<image::DynamicImage> {
        let bytes: &[u8] = match self {
            Self::SurfaceDirt => include_bytes!("../../../assets/textures/surface_dirt.png"),
            Self::SurfaceGrass => include_bytes!("../../../assets/textures/surface_grass.png"),
            Self::SurfaceGravel => include_bytes!("../../../assets/textures/surface_gravel.png"),
            Self::SurfaceSand => include_bytes!("../../../assets/textures/surface_sand.png"),
            Self::SurfaceStone => include_bytes!("../../../assets/textures/surface_stone.png"),
            Self::MixGrass => include_bytes!("../../../assets/textures/mix_grass.png"),
            Self::Dandelion => include_bytes!("../../../assets/textures/dandelion.png"),
            Self::FallenBranch => include_bytes!("../../../assets/textures/fallen_branch.png"),
            Self::FallenLeaves => include_bytes!("../../../assets/textures/fallen_leaves.png"),
            Self::MixPebbles => include_bytes!("../../../assets/textures/mix_pebbles.png"),
            Self::Player => include_bytes!("../../../assets/textures/frame.png"),
            Self::OakTree => include_bytes!("../../../assets/textures/oak_tree.png"),
            Self::BirchTree => include_bytes!("../../../assets/textures/birch_tree.png"),
            Self::DyingTree => include_bytes!("../../../assets/textures/dying_tree.png"),
            Self::FallenTree => include_bytes!("../../../assets/textures/fallen_tree.png"),
            Self::MixRock => include_bytes!("../../../assets/textures/mix_rock.png"),
        };

        image::load_from_memory(bytes)
    }

    pub fn block_size(&self) -> (u32, u32) {
        match self {
            Self::SurfaceDirt => (1, 1),
            Self::SurfaceGrass => (1, 1),
            Self::SurfaceGravel => (1, 1),
            Self::SurfaceSand => (1, 1),
            Self::SurfaceStone => (1, 1),
            Self::MixGrass => (1, 1),
            Self::Dandelion => (1, 1),
            Self::FallenBranch => (1, 1),
            Self::FallenLeaves => (1, 1),
            Self::MixPebbles => (1, 1),
            Self::Player => (1, 2),
            Self::OakTree => (4, 6),
            Self::BirchTree => (4, 6),
            Self::DyingTree => (4, 6),
            Self::FallenTree => (4, 2),
            Self::MixRock => (2, 2),
        }
    }

    pub fn atlas_option(&self) -> UnitAtlasOption {
        match self {
            Self::SurfaceDirt => UnitAtlasOption::Continuous,
            Self::SurfaceGrass => UnitAtlasOption::Continuous,
            Self::SurfaceGravel => UnitAtlasOption::Continuous,
            Self::SurfaceSand => UnitAtlasOption::Continuous,
            Self::SurfaceStone => UnitAtlasOption::Continuous,
            Self::MixGrass => UnitAtlasOption::Single,
            Self::Dandelion => UnitAtlasOption::Single,
            Self::FallenBranch => UnitAtlasOption::Single,
            Self::FallenLeaves => UnitAtlasOption::Single,
            Self::MixPebbles => UnitAtlasOption::Single,
            Self::Player => UnitAtlasOption::Single,
            Self::OakTree => UnitAtlasOption::Single,
            Self::BirchTree => UnitAtlasOption::Single,
            Self::DyingTree => UnitAtlasOption::Single,
            Self::FallenTree => UnitAtlasOption::Single,
            Self::MixRock => UnitAtlasOption::Single,
        }
    }
}
