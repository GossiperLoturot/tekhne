use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Pos3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds3D<T> {
    pub min: Pos3D<T>,
    pub max: Pos3D<T>,
}

impl<T> Bounds3D<T> {
    pub fn new(min: Pos3D<T>, max: Pos3D<T>) -> Self {
        Self { min, max }
    }
}

impl<T: PartialOrd> Bounds3D<T> {
    pub fn inclusive_contains(&self, point: &Pos3D<T>) -> bool {
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
    pub pos: Pos3D<i32>,
    pub resource_name: String,
}
