use crate::model::*;
use glam::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct UnitService {
    units: HashMap<String, Unit>,
}

impl UnitService {
    pub fn add_unit(&mut self, unit: Unit) {
        if self.units.contains_key(&unit.id) {
            panic!("unit is already existed at id {:?}", unit.id);
        }

        self.units.insert(unit.id.clone(), unit);
    }

    pub fn remove_unit(&mut self, id: &str) {
        if !self.units.contains_key(id) {
            panic!("unit is not found at id {:?}", id);
        }

        self.units.remove(id);
    }

    pub fn get_unit(&self, id: &str) -> Option<&Unit> {
        self.units.get(id)
    }

    pub fn get_units(&self, bounds: Bounds<Vec3A>) -> Vec<&Unit> {
        let mut units = vec![];

        for unit in self.units.values() {
            if bounds.inclusive_contains(&unit.pos) {
                units.push(unit);
            }
        }

        units
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_unit() {
        let mut service = UnitService::default();
        service.add_unit(Unit::new(
            "TEST_UNIT_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            ResourceKind::SurfaceDirt,
        ));

        let unit = service.get_unit("TEST_UNIT_ID").unwrap();
        assert_eq!(unit.pos, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.resource_kind, ResourceKind::SurfaceDirt);
    }

    #[test]
    fn remove_unit() {
        let mut service = UnitService::default();
        service.add_unit(Unit::new(
            "TEST_UNIT_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            ResourceKind::SurfaceDirt,
        ));
        service.remove_unit("TEST_UNIT_ID");

        let is_none = service.get_unit("TEST_UNIT_ID").is_none();
        assert!(is_none);
    }

    #[test]
    fn set_bounds_before_fill_data() {
        let mut service = UnitService::default();
        service.add_unit(Unit::new(
            "TEST_UNIT_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            ResourceKind::SurfaceDirt,
        ));
        service.add_unit(Unit::new(
            "TEST_OTHER_UNIT_ID".to_string(),
            Vec3A::new(-1.0, -1.0, -1.0),
            ResourceKind::SurfaceGrass,
        ));

        let units = service.get_units(Bounds::new(
            Vec3A::new(0.0, 0.0, 0.0),
            Vec3A::new(8.0, 8.0, 8.0),
        ));
        assert_eq!(units.len(), 1);
        let unit = units.get(0).unwrap();
        assert_eq!(unit.id, "TEST_UNIT_ID".to_string());
        assert_eq!(unit.pos, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.resource_kind, ResourceKind::SurfaceDirt);
    }
}
