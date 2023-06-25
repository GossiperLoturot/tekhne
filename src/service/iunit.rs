use crate::model::*;
use glam::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct IUnitService {
    iunits: HashMap<IVec3, IUnit>,
}

impl IUnitService {
    pub fn add_iunit(&mut self, iunit: IUnit) {
        if self.iunits.contains_key(&iunit.pos) {
            panic!("iunit is already existed at pos {:?}.", iunit.pos);
        }

        self.iunits.insert(iunit.pos, iunit);
    }

    pub fn remove_iunit(&mut self, pos: IVec3) {
        if !self.iunits.contains_key(&pos) {
            panic!("iunit is not found at pos {:?}", pos);
        }

        self.iunits.remove(&pos);
    }

    pub fn get_iunit(&self, pos: IVec3) -> Option<&IUnit> {
        self.iunits.get(&pos)
    }

    pub fn get_iunits(&self, bounds: Bounds<IVec3>) -> Vec<&IUnit> {
        let mut iunits = vec![];

        for x in bounds.min.x..=bounds.max.x {
            for y in bounds.min.y..=bounds.max.y {
                for z in bounds.min.z..=bounds.max.z {
                    if let Some(iunit) = self.iunits.get(&IVec3::new(x, y, z)) {
                        iunits.push(iunit);
                    }
                }
            }
        }

        iunits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_iunit() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(
            IVec3::new(0, 0, 0),
            "TEST_RESOURCE_NAME".to_string(),
        ));

        let iunit = service.get_iunit(IVec3::new(0, 0, 0)).unwrap();
        assert_eq!(iunit.pos, IVec3::new(0, 0, 0));
        assert_eq!(iunit.resource_name, "TEST_RESOURCE_NAME");
    }

    #[test]
    fn remove_iunit() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(
            IVec3::new(0, 0, 0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.remove_iunit(IVec3::new(0, 0, 0));

        let is_none = service.get_iunit(IVec3::new(0, 0, 0)).is_none();
        assert!(is_none);
    }

    #[test]
    fn set_bounds_before_fill_data() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(
            IVec3::new(0, 0, 0),
            "TEST_RESOURCE_NAME".to_string(),
        ));
        service.add_iunit(IUnit::new(
            IVec3::new(-1, -1, -1),
            "TEST_OTHER_RESOURCE_NAME".to_string(),
        ));

        let iunits = service.get_iunits(Bounds::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8)));
        assert_eq!(iunits.len(), 1);
        let iunit = iunits.get(0).unwrap();
        assert_eq!(iunit.pos, IVec3::new(0, 0, 0));
        assert_eq!(iunit.resource_name, "TEST_RESOURCE_NAME".to_string());
    }
}
