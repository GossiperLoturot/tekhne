use glam::{IVec3, Vec3A};

#[derive(Debug, Clone, Copy)]
pub struct IBounds3 {
    pub min: IVec3,
    pub max: IVec3,
}

impl IBounds3 {
    pub fn new(min: IVec3, max: IVec3) -> Self {
        Self { min, max }
    }

    pub fn inclusive_contains(&self, point: &IVec3) -> bool {
        point.x < self.min.x
            || self.max.x < point.x
            || point.y < self.min.y
            || self.max.y < point.y
            || point.z < self.min.z
            || self.max.z < point.z
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub pos: IVec3,
    pub resource_name: String,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: String,
    pub pos: Vec3A,
    pub resource_name: String,
}
