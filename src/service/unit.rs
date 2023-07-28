use crate::model::*;
use ahash::{AHashMap, HashSet};
use glam::*;
use uuid::Uuid;

#[derive(Default)]
pub struct UnitService {
    units: AHashMap<Uuid, Unit>,
    index_table: AHashMap<IVec3, HashSet<Uuid>>,
}

impl UnitService {
    const GRID_SIZE: f32 = 32.0;

    pub fn add_unit(&mut self, unit: Unit) -> Option<Unit> {
        if !self.units.contains_key(&unit.id) {
            let aabb = unit.aabb().grid_partition(Self::GRID_SIZE);
            for x in aabb.min.x..=aabb.max.x {
                for y in aabb.min.y..=aabb.max.y {
                    for z in aabb.min.z..=aabb.max.z {
                        self.index_table
                            .entry(IVec3::new(x, y, z))
                            .or_default()
                            .insert(unit.id);
                    }
                }
            }

            self.units.insert(unit.id, unit)
        } else {
            None
        }
    }

    pub fn remove_unit(&mut self, id: &Uuid) -> Option<Unit> {
        if let Some(unit) = self.units.get(id) {
            let aabb = unit.aabb().grid_partition(Self::GRID_SIZE);
            for x in aabb.min.x..=aabb.max.x {
                for y in aabb.min.y..=aabb.max.y {
                    for z in aabb.min.z..=aabb.max.z {
                        self.index_table
                            .entry(IVec3::new(x, y, z))
                            .or_default()
                            .remove(&unit.id);
                    }
                }
            }

            self.units.remove(id)
        } else {
            None
        }
    }

    pub fn get_unit(&self, id: &Uuid) -> Option<&Unit> {
        self.units.get(id)
    }

    pub fn get_units(&self, aabb: Aabb3A) -> Vec<&Unit> {
        let mut units = vec![];

        let grid_aabb = aabb.grid_partition(Self::GRID_SIZE);
        for x in grid_aabb.min.x..=grid_aabb.max.x {
            for y in grid_aabb.min.y..=grid_aabb.max.y {
                for z in grid_aabb.min.z..=grid_aabb.max.z {
                    if let Some(ids) = self.index_table.get(&IVec3::new(x, y, z)) {
                        ids.iter()
                            .filter_map(|id| self.units.get(id))
                            .filter(|unit| aabb.intersects(&unit.aabb()))
                            .for_each(|unit| units.push(unit));
                    }
                }
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
        service.add_unit(Unit::new(id, Vec3A::new(0.0, 0.0, 0.0), UnitKind::OakTree));

        let unit = service.get_unit(&id).unwrap();
        assert_eq!(unit.position, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.kind, UnitKind::OakTree);
    }

    #[test]
    fn remove_unit() {
        let mut service = UnitService::default();

        let id = Uuid::new_v4();
        service.add_unit(Unit::new(id, Vec3A::new(0.0, 0.0, 0.0), UnitKind::OakTree));
        service.remove_unit(&id);

        let is_none = service.get_unit(&id).is_none();
        assert!(is_none);
    }

    #[test]
    fn set_aabb_before_fill_data() {
        let mut service = UnitService::default();

        let id = Uuid::new_v4();
        service.add_unit(Unit::new(id, Vec3A::new(0.0, 0.0, 0.0), UnitKind::OakTree));

        let other_id = Uuid::new_v4();
        service.add_unit(Unit::new(
            other_id,
            Vec3A::new(-4.0, -4.0, -4.0),
            UnitKind::OakTree,
        ));

        let units = service.get_units(Aabb3A::new(
            Vec3A::new(0.0, 0.0, 0.0),
            Vec3A::new(8.0, 8.0, 8.0),
        ));
        assert_eq!(units.len(), 1);
        let unit = units.get(0).unwrap();
        assert_eq!(unit.id, id);
        assert_eq!(unit.position, Vec3A::new(0.0, 0.0, 0.0));
        assert_eq!(unit.kind, UnitKind::OakTree);
    }
}
