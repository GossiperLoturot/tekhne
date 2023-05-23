use glam::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IBounds3 {
    pub min: IVec3,
    pub max: IVec3,
}

impl IBounds3 {
    pub fn new(min: IVec3, max: IVec3) -> Self {
        Self { min, max }
    }

    pub fn inclusive_contains(&self, point: &IVec3) -> bool {
        let out_of_bounds = point.x < self.min.x
            || self.max.x < point.x
            || point.y < self.min.y
            || self.max.y < point.y
            || point.z < self.min.z
            || self.max.z < point.z;
        !out_of_bounds
    }
}

#[derive(Debug, Clone)]
pub struct IUnit {
    pub pos: IVec3,
    pub resource_name: String,
}

impl IUnit {
    pub fn new(pos: IVec3, resource_name: String) -> Self {
        Self { pos, resource_name }
    }
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub id: String,
    pub pos: Vec3A,
    pub resource_name: String,
}

impl Unit {
    pub fn new(id: String, pos: Vec3A, resource_name: String) -> Self {
        Self {
            id,
            pos,
            resource_name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub position: Vec3A,
    pub view_area: IBounds3,
    pub zoom: f32,
}

impl Player {
    pub fn new(position: Vec3A, view_area: IBounds3, zoom: f32) -> Self {
        Self {
            position,
            view_area,
            zoom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inclusive_contains_in_bound() {
        let bounds = IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(1, 2, 3);
        assert!(bounds.inclusive_contains(&point));
    }

    #[test]
    fn inclusive_contains_on_border_a() {
        let bounds = IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(0, 0, 0);
        assert!(bounds.inclusive_contains(&point));
    }

    #[test]
    fn inclusive_contains_on_border_b() {
        let bounds = IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(8, 8, 8);
        assert!(bounds.inclusive_contains(&point));
    }

    #[test]
    fn inclusive_contains_out_of_bound() {
        let bounds = IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(9, 10, 11);
        assert!(!bounds.inclusive_contains(&point));
    }
}
