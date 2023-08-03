use crate::model::*;
use glam::*;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum UnitShape {
    Block,
    Top,
    Bottom,
    Quad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum TextureOption {
    Single,
    Continuous,
}

impl UnitKind {
    pub fn shape(&self) -> UnitShape {
        match self {
            Self::SurfaceDirt
            | Self::SurfaceGrass
            | Self::SurfaceGravel
            | Self::SurfaceSand
            | Self::SurfaceStone => UnitShape::Top,
            Self::MixGrass
            | Self::Dandelion
            | Self::FallenBranch
            | Self::FallenLeaves
            | Self::MixPebbles => UnitShape::Bottom,
            Self::Player
            | Self::OakTree
            | Self::BirchTree
            | Self::DyingTree
            | Self::FallenTree
            | Self::MixRock => UnitShape::Quad,
        }
    }

    pub fn shape_size(&self) -> Aabb3A {
        match self {
            Self::Player => Aabb3A::new(Vec3A::new(-0.5, 0.0, 0.0), Vec3A::new(0.5, 0.0, 2.0)),
            Self::OakTree => Aabb3A::new(Vec3A::new(-2.0, 0.0, 0.0), Vec3A::new(2.0, 0.0, 6.0)),
            Self::BirchTree => Aabb3A::new(Vec3A::new(-2.0, 0.0, 0.0), Vec3A::new(2.0, 0.0, 6.0)),
            Self::DyingTree => Aabb3A::new(Vec3A::new(-2.0, 0.0, 0.0), Vec3A::new(2.0, 0.0, 6.0)),
            Self::FallenTree => Aabb3A::new(Vec3A::new(-2.0, 0.0, 0.0), Vec3A::new(2.0, 0.0, 2.0)),
            Self::MixRock => Aabb3A::new(Vec3A::new(-1.0, 0.0, 0.0), Vec3A::new(1.0, 0.0, 2.0)),
            _ => Aabb3A::new(Vec3A::new(-0.5, -0.5, -0.5), Vec3A::new(0.5, 0.5, 0.5)),
        }
    }

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

    pub fn texture_option(&self) -> TextureOption {
        match self {
            UnitKind::SurfaceDirt
            | UnitKind::SurfaceGrass
            | UnitKind::SurfaceGravel
            | UnitKind::SurfaceSand
            | UnitKind::SurfaceStone => TextureOption::Continuous,
            _ => TextureOption::Single,
        }
    }
}
