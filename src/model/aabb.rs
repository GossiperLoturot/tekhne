use glam::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct IAabb3 {
    pub min: IVec3,
    pub max: IVec3,
}

impl IAabb3 {
    pub fn new(min: IVec3, max: IVec3) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, point: &IVec3) -> bool {
        let is_outside = point.x < self.min.x
            || self.max.x < point.x
            || point.y < self.min.y
            || self.max.y < point.y
            || point.z < self.min.z
            || self.max.z < point.z;
        !is_outside
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let self_size = self.max - self.min;
        let self_origin = self.min + self.max;

        let other_size = other.max - other.min;
        let other_origin = other.min + other.max;

        let norm = (other_origin - self_origin).abs();
        let size = self_size + other_size;
        let is_outside = size.x < norm.x || size.y < norm.y || size.z < norm.z;
        !is_outside
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Aabb3A {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl Aabb3A {
    pub fn new(min: Vec3A, max: Vec3A) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, point: &Vec3A) -> bool {
        let is_outside = point.x < self.min.x
            || self.max.x < point.x
            || point.y < self.min.y
            || self.max.y < point.y
            || point.z < self.min.z
            || self.max.z < point.z;
        !is_outside
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let self_size = self.max - self.min;
        let self_origin = self.min + self.max;

        let other_size = other.max - other.min;
        let other_origin = other.min + other.max;

        let norm = (other_origin - self_origin).abs();
        let size = self_size + other_size;
        let is_outside = size.x < norm.x || size.y < norm.y || size.z < norm.z;
        !is_outside
    }

    pub fn grid_partition(&self, grid_size: f32) -> IAabb3 {
        let min = (self.min / grid_size).floor().as_ivec3();
        let max = (self.max / grid_size).floor().as_ivec3();
        IAabb3 { min, max }
    }

    pub fn as_iaabb3(&self) -> IAabb3 {
        IAabb3 {
            min: self.min.as_ivec3(),
            max: self.max.as_ivec3(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_inside() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(1, 2, 3);
        assert!(aabb.contains(&point));
    }

    #[test]
    fn contains_on_border_a() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(0, 0, 0);
        assert!(aabb.contains(&point));
    }

    #[test]
    fn contains_on_border_b() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(8, 8, 8);
        assert!(aabb.contains(&point));
    }

    #[test]
    fn contains_outside() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(9, 10, 11);
        assert!(!aabb.contains(&point));
    }

    #[test]
    fn intersects_inside() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let other_aabb = IAabb3::new(IVec3::new(2, 2, 2), IVec3::new(6, 6, 6));
        assert!(aabb.intersects(&other_aabb));
    }

    #[test]
    fn intersects_a() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let other_aabb = IAabb3::new(IVec3::new(-4, -4, -4), IVec3::new(4, 4, 4));
        assert!(aabb.intersects(&other_aabb));
    }

    #[test]
    fn intersects_b() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let other_aabb = IAabb3::new(IVec3::new(-8, 2, 2), IVec3::new(2, 6, 6));
        assert!(aabb.intersects(&other_aabb));
    }

    #[test]
    fn intersects_on_border_a() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let other_aabb = IAabb3::new(IVec3::new(-8, 0, 0), IVec3::new(0, 8, 8));
        assert!(aabb.intersects(&other_aabb));
    }

    #[test]
    fn intersects_on_border_b() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let other_aabb = IAabb3::new(IVec3::new(-8, -8, -8), IVec3::new(0, 0, 0));
        assert!(aabb.intersects(&other_aabb));
    }

    #[test]
    fn intersects_outside_a() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let other_aabb = IAabb3::new(IVec3::new(-8, -8, -8), IVec3::new(-4, -4, -4));
        assert!(!aabb.intersects(&other_aabb));
    }

    #[test]
    fn intersects_outside_b() {
        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let other_aabb = IAabb3::new(IVec3::new(-8, 2, 2), IVec3::new(-4, 6, 6));
        assert!(!aabb.intersects(&other_aabb));
    }
}
