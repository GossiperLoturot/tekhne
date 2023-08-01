use super::Aabb2;
use glam::*;
use strum::EnumIter;

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Block,
    Top,
    Bottom,
    Quad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Kind {
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

impl Kind {
    pub fn shape(&self) -> Shape {
        match self {
            Self::SurfaceDirt
            | Self::SurfaceGrass
            | Self::SurfaceGravel
            | Self::SurfaceSand
            | Self::SurfaceStone => Shape::Top,
            Self::MixGrass
            | Self::Dandelion
            | Self::FallenBranch
            | Self::FallenLeaves
            | Self::MixPebbles => Shape::Bottom,
            Self::Player
            | Self::OakTree
            | Self::BirchTree
            | Self::DyingTree
            | Self::FallenTree
            | Self::MixRock => Shape::Quad,
        }
    }

    pub fn shape_size(&self) -> Aabb2 {
        match self {
            Self::Player => Aabb2::from_element(-0.5, 0.0, 0.5, 2.0),
            Self::OakTree => Aabb2::from_element(-2.0, 0.0, 2.0, 6.0),
            Self::BirchTree => Aabb2::from_element(-2.0, 0.0, 2.0, 6.0),
            Self::DyingTree => Aabb2::from_element(-2.0, 0.0, 2.0, 6.0),
            Self::FallenTree => Aabb2::from_element(-2.0, 0.0, 2.0, 2.0),
            Self::MixRock => Aabb2::from_element(-1.0, 0.0, 1.0, 2.0),
            _ => Aabb2::from_element(-0.5, -0.5, 0.5, 0.5),
        }
    }

    pub fn texture(&self) -> image::DynamicImage {
        let bytes: &[u8] = match self {
            Self::SurfaceDirt => include_bytes!("../../assets/textures/surface_dirt.png"),
            Self::SurfaceGrass => include_bytes!("../../assets/textures/surface_grass.png"),
            Self::SurfaceGravel => include_bytes!("../../assets/textures/surface_gravel.png"),
            Self::SurfaceSand => include_bytes!("../../assets/textures/surface_sand.png"),
            Self::SurfaceStone => include_bytes!("../../assets/textures/surface_stone.png"),
            Self::MixGrass => include_bytes!("../../assets/textures/mix_grass.png"),
            Self::Dandelion => include_bytes!("../../assets/textures/dandelion.png"),
            Self::FallenBranch => include_bytes!("../../assets/textures/fallen_branch.png"),
            Self::FallenLeaves => include_bytes!("../../assets/textures/fallen_leaves.png"),
            Self::MixPebbles => include_bytes!("../../assets/textures/mix_pebbles.png"),
            Self::Player => include_bytes!("../../assets/textures/frame.png"),
            Self::OakTree => include_bytes!("../../assets/textures/oak_tree.png"),
            Self::BirchTree => include_bytes!("../../assets/textures/birch_tree.png"),
            Self::DyingTree => include_bytes!("../../assets/textures/dying_tree.png"),
            Self::FallenTree => include_bytes!("../../assets/textures/fallen_tree.png"),
            Self::MixRock => include_bytes!("../../assets/textures/mix_rock.png"),
        };

        image::load_from_memory(bytes).expect("failed to load texture")
    }

    pub fn texture_size(&self) -> (u32, u32) {
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
}
