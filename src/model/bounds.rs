use glam::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Bounds<T> {
    pub min: T,
    pub max: T,
}

impl<T> Bounds<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

impl Bounds<IVec3> {
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

impl Bounds<Vec3A> {
    pub fn inclusive_contains(&self, point: &Vec3A) -> bool {
        let out_of_bounds = point.x < self.min.x
            || self.max.x < point.x
            || point.y < self.min.y
            || self.max.y < point.y
            || point.z < self.min.z
            || self.max.z < point.z;
        !out_of_bounds
    }
}

impl From<Bounds<Vec3A>> for Bounds<IVec3> {
    fn from(value: Bounds<Vec3A>) -> Self {
        Self {
            min: value.min.as_ivec3(),
            max: value.max.as_ivec3(),
        }
    }
}

impl From<Bounds<IVec3>> for Bounds<Vec3A> {
    fn from(value: Bounds<IVec3>) -> Self {
        Self {
            min: value.min.as_vec3a(),
            max: value.max.as_vec3a(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inclusive_contains_in_bound() {
        let bounds = Bounds::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(1, 2, 3);
        assert!(bounds.inclusive_contains(&point));
    }

    #[test]
    fn inclusive_contains_on_border_a() {
        let bounds = Bounds::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(0, 0, 0);
        assert!(bounds.inclusive_contains(&point));
    }

    #[test]
    fn inclusive_contains_on_border_b() {
        let bounds = Bounds::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(8, 8, 8);
        assert!(bounds.inclusive_contains(&point));
    }

    #[test]
    fn inclusive_contains_out_of_bound() {
        let bounds = Bounds::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        let point = IVec3::new(9, 10, 11);
        assert!(!bounds.inclusive_contains(&point));
    }
}
