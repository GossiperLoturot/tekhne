use super::{shape::UnitVertex, texture::UnitAtlasOption};
use crate::model::*;
use glam::*;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum UnitShapeItem {
    Block,
    Top,
    Bottom,
    Quad,
    Quad1x2,
    Quad2x2,
    Quad4x2,
    Quad4x6,
}

impl From<UnitKind> for UnitShapeItem {
    fn from(value: UnitKind) -> Self {
        match value {
            UnitKind::SurfaceDirt
            | UnitKind::SurfaceGrass
            | UnitKind::SurfaceGravel
            | UnitKind::SurfaceSand
            | UnitKind::SurfaceStone => Self::Top,
            UnitKind::MixGrass
            | UnitKind::Dandelion
            | UnitKind::FallenBranch
            | UnitKind::FallenLeaves
            | UnitKind::MixPebbles => Self::Bottom,
            UnitKind::Player => Self::Quad1x2,
            UnitKind::OakTree | UnitKind::BirchTree | UnitKind::DyingTree => Self::Quad4x6,
            UnitKind::FallenTree => Self::Quad4x2,
            UnitKind::MixRock => Self::Quad2x2,
        }
    }
}

impl UnitShapeItem {
    #[rustfmt::skip]
    pub fn vertices(&self) -> Vec<UnitVertex> {
        match self {
            UnitShapeItem::Block => vec![
                UnitVertex { position: [-0.5, -0.5, -0.5], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 0.5, -0.5, -0.5], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 0.5,  0.5,  0.5], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-0.5,  0.5,  0.5], texcoord: [0.0, 0.0] },
            ],                                           
            UnitShapeItem::Top => vec![
                UnitVertex { position: [-0.5, -0.5,  0.5], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 0.5, -0.5,  0.5], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 0.5,  0.5,  0.5], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-0.5,  0.5,  0.5], texcoord: [0.0, 0.0] },
            ],                                           
            UnitShapeItem::Bottom => vec![
                UnitVertex { position: [-0.5, -0.5, -0.5], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 0.5, -0.5, -0.5], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 0.5,  0.5, -0.5], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-0.5,  0.5, -0.5], texcoord: [0.0, 0.0] },
            ],                                           
            UnitShapeItem::Quad => vec![
                UnitVertex { position: [-0.5,  0.0,  0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 0.5,  0.0,  0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 0.5,  0.0,  1.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-0.5,  0.0,  1.0], texcoord: [0.0, 0.0] },
            ],
            UnitShapeItem::Quad1x2 => vec![
                UnitVertex { position: [-0.5,  0.0,  0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 0.5,  0.0,  0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 0.5,  0.0,  2.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-0.5,  0.0,  2.0], texcoord: [0.0, 0.0] },
            ],
            UnitShapeItem::Quad2x2 => vec![
                UnitVertex { position: [-1.0,  0.0,  0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 1.0,  0.0,  0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 1.0,  0.0,  2.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-1.0,  0.0,  2.0], texcoord: [0.0, 0.0] },
            ],
            UnitShapeItem::Quad4x2 => vec![
                UnitVertex { position: [-2.0,  0.0,  0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 2.0,  0.0,  0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 2.0,  0.0,  2.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-2.0,  0.0,  2.0], texcoord: [0.0, 0.0] },
            ],
            UnitShapeItem::Quad4x6 => vec![
                UnitVertex { position: [-2.0,  0.0,  0.0], texcoord: [0.0, 1.0] },
                UnitVertex { position: [ 2.0,  0.0,  0.0], texcoord: [1.0, 1.0] },
                UnitVertex { position: [ 2.0,  0.0,  6.0], texcoord: [1.0, 0.0] },
                UnitVertex { position: [-2.0,  0.0,  6.0], texcoord: [0.0, 0.0] },
            ],
        }
    }

    pub fn indices(&self) -> Vec<u16> {
        vec![0, 1, 2, 2, 3, 0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum UnitAtlasItem {
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

impl From<UnitKind> for UnitAtlasItem {
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

impl UnitAtlasItem {
    pub fn texture(&self) -> Option<image::DynamicImage> {
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

        image::load_from_memory(bytes).ok()
    }

    pub fn block_size(&self) -> (u32, u32) {
        match self {
            Self::Player => (1, 2),
            Self::OakTree => (4, 6),
            Self::BirchTree => (4, 6),
            Self::DyingTree => (4, 6),
            Self::FallenTree => (4, 2),
            Self::MixRock => (2, 2),
            _ => (1, 1),
        }
    }

    pub fn atlas_option(&self) -> UnitAtlasOption {
        match self {
            Self::SurfaceDirt
            | Self::SurfaceGrass
            | Self::SurfaceGravel
            | Self::SurfaceSand
            | Self::SurfaceStone => UnitAtlasOption::Continuous,
            _ => UnitAtlasOption::Single,
        }
    }
}
