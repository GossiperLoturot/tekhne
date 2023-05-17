use crate::models::*;
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

    pub fn get_units(&self, bounds: IBounds3) -> Vec<&Unit> {
        let mut units = vec![];

        for unit in self.units.values() {
            if bounds.inclusive_contains(&unit.pos.round().as_ivec3()) {
                units.push(unit);
            }
        }

        units
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::*;

    #[test]
    fn add_unit() {
        let mut service = UnitService::default();
        service.add_unit(Unit::new(
            "TEST_UNIT_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            "TEST_RESOURCE_NAME".to_string(),
        ));

        let unit = service.get_unit("TEST_UNIT_ID").unwrap();
        assert_eq!(unit.pos, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.resource_name, "TEST_RESOURCE_NAME");
    }

    #[test]
    fn remove_unit() {
        let mut service = UnitService::default();
        service.add_unit(Unit::new(
            "TEST_UNIT_ID".to_string(),
            Vec3A::new(0.0, 0.0, 0.0),
            "TEST_RESOURCE_NAME".to_string(),
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
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.add_unit(Unit::new(
            "TEST_OTHER_UNIT_ID".to_string(),
            Vec3A::new(-1.0, -1.0, -1.0),
            "TEST_OTHER_RESOURCE_NAME".to_string(),
        ));

        let units = service.get_units(IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8)));
        assert_eq!(units.len(), 1);
        let unit = units.get(0).unwrap();
        assert_eq!(unit.id, "TEST_UNIT_ID".to_string());
        assert_eq!(unit.pos, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.resource_name, "TEST_RESOURCE_NAME".to_string());
    }
}
