use super::UnitVertex;
use crate::model::*;
use strum::EnumIter;

pub enum UnitModelItem {
    Block,
    TopPlane,
    BottomPlane,
    Billboard(f32, f32),
}

impl UnitModelItem {
    #[rustfmt::skip]
    pub fn vertices(&self) -> Vec<UnitVertex> {
        match self {
            UnitModelItem::Block => vec![
                UnitVertex { position: [-0.5, -0.5, -0.5], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 0.5, -0.5, -0.5], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 0.5,  0.5,  0.5], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-0.5,  0.5,  0.5], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::TopPlane => vec![
                UnitVertex { position: [-0.5, -0.5,  0.5], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 0.5, -0.5,  0.5], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 0.5,  0.5,  0.5], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-0.5,  0.5,  0.5], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::BottomPlane => vec![
                UnitVertex { position: [-0.5, -0.5, -0.49], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 0.5, -0.5, -0.49], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 0.5,  0.5, -0.49], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-0.5,  0.5, -0.49], texcoord: [0.0, 0.0] },
            ],
            UnitModelItem::Billboard(width, height) => vec![
                UnitVertex { position: [-0.5 * width, 0.0, 0.0 * height], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 0.5 * width, 0.0, 0.0 * height], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 0.5 * width, 0.0, 1.0 * height], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-0.5 * width, 0.0, 1.0 * height], texcoord: [0.0, 0.0] },
            ],
        }
    }

    pub fn indices(&self) -> Vec<u32> {
        match self {
            UnitModelItem::Block => vec![0, 1, 2, 2, 3, 0],
            UnitModelItem::TopPlane => vec![0, 1, 2, 2, 3, 0],
            UnitModelItem::BottomPlane => vec![0, 1, 2, 2, 3, 0],
            UnitModelItem::Billboard(_, _) => vec![0, 1, 2, 2, 3, 0],
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
            UnitKind::Player => Self::Billboard(1.0, 2.0),
            UnitKind::OakTree => Self::Billboard(4.0, 6.0),
            UnitKind::BirchTree => Self::Billboard(4.0, 6.0),
            UnitKind::DyingTree => Self::Billboard(4.0, 6.0),
            UnitKind::FallenTree => Self::Billboard(4.0, 2.0),
            UnitKind::MixRock => Self::Billboard(2.0, 2.0),
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
}
