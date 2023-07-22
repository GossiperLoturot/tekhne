use super::Aabb3A;
use glam::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitKind {
    OakTree,
    BirchTree,
    DyingTree,
    FallenTree,
    MixRock,
}

impl UnitKind {
    pub fn entry() -> [Self; 5] {
        [
            Self::OakTree,
            Self::BirchTree,
            Self::DyingTree,
            Self::FallenTree,
            Self::MixRock,
        ]
    }

    pub fn texture(&self) -> Option<image::DynamicImage> {
        let bytes: Option<&[u8]> = match self {
            Self::OakTree => Some(include_bytes!("../../assets/textures/oak_tree.png")),
            Self::BirchTree => Some(include_bytes!("../../assets/textures/birch_tree.png")),
            Self::DyingTree => Some(include_bytes!("../../assets/textures/dying_tree.png")),
            Self::FallenTree => Some(include_bytes!("../../assets/textures/fallen_tree.png")),
            Self::MixRock => Some(include_bytes!("../../assets/textures/mix_rock.png")),
        };

        bytes.and_then(|bytes| image::load_from_memory(bytes).ok())
    }

    pub fn texture_size(&self) -> Option<IVec2> {
        match self {
            Self::OakTree => Some(IVec2::new(4, 8)),
            Self::BirchTree => Some(IVec2::new(4, 8)),
            Self::DyingTree => Some(IVec2::new(4, 8)),
            Self::FallenTree => Some(IVec2::new(4, 8)),
            Self::MixRock => Some(IVec2::new(2, 4)),
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
        true
    }

    pub fn aabb(&self) -> Aabb3A {
        match self.kind {
            UnitKind::OakTree => {
                Aabb3A::splat(self.position + Vec3A::new(0.0, 0.0, 2.0), Vec3A::splat(2.0))
            }
            UnitKind::BirchTree => {
                Aabb3A::splat(self.position + Vec3A::new(0.0, 0.0, 2.0), Vec3A::splat(2.0))
            }
            UnitKind::DyingTree => {
                Aabb3A::splat(self.position + Vec3A::new(0.0, 0.0, 2.0), Vec3A::splat(2.0))
            }
            UnitKind::FallenTree => {
                Aabb3A::splat(self.position + Vec3A::new(0.0, 0.0, 2.0), Vec3A::splat(2.0))
            }
            UnitKind::MixRock => {
                Aabb3A::splat(self.position + Vec3A::new(0.0, 0.0, 1.0), Vec3A::splat(1.0))
            }
        }
    }
}
