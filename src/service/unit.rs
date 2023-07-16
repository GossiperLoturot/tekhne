// TODO: improve performance by grid space partitioning

use crate::model::*;
use ahash::AHashMap;
use glam::*;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct UnitService {
    units: AHashMap<Uuid, Unit>,
}

impl UnitService {
    pub fn add_unit(&mut self, unit: Unit) -> Option<()> {
        if self.units.contains_key(&unit.id) {
            return None;
        }

        self.units.insert(unit.id.clone(), unit);
        Some(())
    }

    pub fn remove_unit(&mut self, id: &Uuid) -> Option<()> {
        if !self.units.contains_key(id) {
            return None;
        }

        self.units.remove(id);
        Some(())
    }

    pub fn get_unit(&self, id: &Uuid) -> Option<&Unit> {
        self.units.get(id)
    }

    pub fn get_units(&self, aabb: Aabb3A) -> Vec<&Unit> {
        let mut units = vec![];

        for unit in self.units.values() {
            let unit_aabb = Aabb3A::splat(unit.position, unit.resource_kind.scale() * 0.5);

            if aabb.intersects(&unit_aabb) {
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

        let id = Uuid::new_v4();
        service.add_unit(Unit::new(
            id,
            Vec3A::new(0.0, 0.0, 0.0),
            ResourceKind::SurfaceDirt,
        ));

        let unit = service.get_unit(&id).unwrap();
        assert_eq!(unit.position, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.resource_kind, ResourceKind::SurfaceDirt);
    }

    #[test]
    fn remove_unit() {
        let mut service = UnitService::default();

        let id = Uuid::new_v4();
        service.add_unit(Unit::new(
            id,
            Vec3A::new(0.0, 0.0, 0.0),
            ResourceKind::SurfaceDirt,
        ));
        service.remove_unit(&id);

        let is_none = service.get_unit(&id).is_none();
        assert!(is_none);
    }

    #[test]
    fn set_aabb_before_fill_data() {
        let mut service = UnitService::default();

        let id = Uuid::new_v4();
        service.add_unit(Unit::new(
            id,
            Vec3A::new(0.0, 0.0, 0.0),
            ResourceKind::SurfaceDirt,
        ));

        let other_id = Uuid::new_v4();
        service.add_unit(Unit::new(
            other_id,
            Vec3A::new(-2.0, -2.0, -2.0),
            ResourceKind::SurfaceGrass,
        ));

        let units = service.get_units(Aabb3A::new(
            Vec3A::new(0.0, 0.0, 0.0),
            Vec3A::new(8.0, 8.0, 8.0),
        ));
        assert_eq!(units.len(), 1);
        let unit = units.get(0).unwrap();
        assert_eq!(unit.id, id);
        assert_eq!(unit.position, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.resource_kind, ResourceKind::SurfaceDirt);
    }
}
