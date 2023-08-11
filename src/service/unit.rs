use crate::model::*;
use ahash::{AHashMap, AHashSet};
use glam::*;
use slab::Slab;

#[derive(Default)]
pub struct UnitService {
    units: Slab<Unit>,
    index_table: AHashMap<IVec3, AHashSet<usize>>,
}

impl UnitService {
    const GRID_SIZE: f32 = 32.0;

    pub fn add_unit(&mut self, unit: Unit) -> usize {
        let id = self.units.vacant_key();

        let grid_aabb = (unit.aabb() / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb.iter().for_each(|index| {
            self.index_table.entry(index).or_default().insert(id);
        });

        self.units.insert(unit)
    }

    pub fn remove_unit(&mut self, id: usize) -> Option<Unit> {
        let unit = self.units.try_remove(id);

        if let Some(unit) = unit {
            let grid_aabb = (unit.aabb() / Self::GRID_SIZE).floor().as_iaabb3();
            grid_aabb.iter().for_each(|index| {
                self.index_table.entry(index).or_default().remove(&id);
            });

            Some(unit)
        } else {
            None
        }
    }

    pub fn get_unit(&self, id: usize) -> Option<&Unit> {
        self.units.get(id)
    }

    pub fn get_units(&self, aabb: Aabb3A) -> Vec<&Unit> {
        let grid_aabb = (aabb / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb
            .iter()
            .filter_map(|grid_index| self.index_table.get(&grid_index))
            .flatten()
            .collect::<AHashSet<_>>()
            .into_iter()
            .filter_map(|id| self.units.get(*id))
            .filter(|unit| aabb.contains(unit.position))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_unit() {
        let mut service = UnitService::default();

        let id = service.add_unit(Unit::new(vec3a(0.0, 0.0, 0.0), UnitKind::Player));

        let unit = service.get_unit(id).unwrap();
        assert_eq!(unit.position, vec3a(0.0, 0.0, 0.0));
        assert_eq!(unit.kind, UnitKind::Player);
    }

    #[test]
    fn remove_unit() {
        let mut service = UnitService::default();

        let id = service.add_unit(Unit::new(vec3a(0.0, 0.0, 0.0), UnitKind::Player));
        service.remove_unit(id);

        assert!(service.get_unit(id).is_none());
    }

    #[test]
    fn set_aabb_before_fill_data() {
        let mut service = UnitService::default();

        service.add_unit(Unit::new(vec3a(0.0, 0.0, 0.0), UnitKind::Player));
        service.add_unit(Unit::new(vec3a(-4.0, -4.0, -4.0), UnitKind::Player));

        let units = service.get_units(aabb3a(vec3a(0.0, 0.0, 0.0), vec3a(8.0, 8.0, 8.0)));
        assert_eq!(units.len(), 1);

        let unit = units.get(0).unwrap();
        assert_eq!(unit.position, vec3a(0.0, 0.0, 0.0));
        assert_eq!(unit.kind, UnitKind::Player);
    }
}
