use super::Aabb3A;
use glam::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitKind {
    UnitTree,
    UnitStone,
}

impl UnitKind {
    pub fn entry() -> [Self; 2] {
        [Self::UnitTree, Self::UnitStone]
    }

    pub fn texture(&self) -> Option<image::DynamicImage> {
        let bytes: Option<&[u8]> = match self {
            Self::UnitTree => Some(include_bytes!("../../assets/textures/frame.png")),
            Self::UnitStone => Some(include_bytes!("../../assets/textures/frame.png")),
        };

        bytes.and_then(|bytes| image::load_from_memory(bytes).ok())
    }

    pub fn texture_size(&self) -> Option<IVec2> {
        match self {
            Self::UnitTree => Some(IVec2::new(4, 8)),
            Self::UnitStone => Some(IVec2::new(1, 2)),
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
            UnitKind::UnitTree => true,
            UnitKind::UnitStone => true,
        }
    }

    pub fn aabb(&self) -> Aabb3A {
        match self.kind {
            UnitKind::UnitTree => Aabb3A::splat(self.position, Vec3A::splat(2.0)),
            UnitKind::UnitStone => Aabb3A::splat(self.position, Vec3A::splat(0.5)),
        }
    }
}
