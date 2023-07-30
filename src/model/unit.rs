use glam::*;
use strum::*;
use uuid::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum UnitKind {
    Player,
    OakTree,
    BirchTree,
    DyingTree,
    FallenTree,
    MixRock,
}

impl UnitKind {
    // return Vec4 (xy: min, zw: max)
    pub fn shape(&self) -> Vec4 {
        match self {
            Self::Player => Vec4::new(-0.5, 0.0, 0.5, 2.0),
            Self::OakTree => Vec4::new(-2.0, 0.0, 2.0, 6.0),
            Self::BirchTree => Vec4::new(-2.0, 0.0, 2.0, 6.0),
            Self::DyingTree => Vec4::new(-2.0, 0.0, 2.0, 6.0),
            Self::FallenTree => Vec4::new(-2.0, 0.0, 2.0, 2.0),
            Self::MixRock => Vec4::new(-1.0, 0.0, 1.0, 2.0),
        }
    }

    pub fn texture(&self) -> Option<image::DynamicImage> {
        let bytes: &[u8] = match self {
            Self::Player => include_bytes!("../../assets/textures/frame.png"),
            Self::OakTree => include_bytes!("../../assets/textures/oak_tree.png"),
            Self::BirchTree => include_bytes!("../../assets/textures/birch_tree.png"),
            Self::DyingTree => include_bytes!("../../assets/textures/dying_tree.png"),
            Self::FallenTree => include_bytes!("../../assets/textures/fallen_tree.png"),
            Self::MixRock => include_bytes!("../../assets/textures/mix_rock.png"),
        };

        image::load_from_memory(bytes).ok()
    }

    pub fn texture_size(&self) -> IVec2 {
        match self {
            Self::Player => IVec2::new(1, 2),
            Self::OakTree => IVec2::new(4, 6),
            Self::BirchTree => IVec2::new(4, 6),
            Self::DyingTree => IVec2::new(4, 6),
            Self::FallenTree => IVec2::new(4, 2),
            Self::MixRock => IVec2::new(2, 2),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: Uuid,
    pub position: Vec3A,
    pub kind: UnitKind,
}

impl Unit {
    pub fn new(id: Uuid, position: Vec3A, kind: UnitKind) -> Self {
        Self { id, position, kind }
    }

    pub fn breakable(&self) -> bool {
        match self.kind {
            UnitKind::Player => false,
            _ => true,
        }
    }
}
