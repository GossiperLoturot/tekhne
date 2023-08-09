use crate::model::*;
use ahash::{AHashMap, AHashSet};
use glam::*;

#[derive(Default)]
pub struct UnitService {
    units: AHashMap<u64, Unit>,
    index_table: AHashMap<IVec3, AHashSet<u64>>,
}

impl UnitService {
    const GRID_SIZE: f32 = 32.0;

    pub fn add_unit(&mut self, unit: Unit) -> Option<&Unit> {
        let id = unit.id;

        if !self.units.contains_key(&unit.id) {
            let grid_aabb = (unit.aabb() / Self::GRID_SIZE).floor().as_iaabb3();
            grid_aabb.iter().for_each(|index| {
                self.index_table.entry(index).or_default().insert(id);
            });

            self.units.insert(id, unit);
            self.units.get(&id)
        } else {
            None
        }
    }

    pub fn remove_unit(&mut self, id: u64) -> Option<Unit> {
        if let Some(unit) = self.units.get(&id) {
            let grid_aabb = (unit.aabb() / Self::GRID_SIZE).floor().as_iaabb3();
            grid_aabb.iter().for_each(|index| {
                self.index_table.entry(index).or_default().remove(&id);
            });

            self.units.remove(&id)
        } else {
            None
        }
    }

    pub fn get_unit(&self, id: u64) -> Option<&Unit> {
        self.units.get(&id)
    }

    pub fn get_units(&self, aabb: Aabb3A) -> Vec<&Unit> {
        let grid_aabb = (aabb / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb
            .iter()
            .filter_map(|grid_index| self.index_table.get(&grid_index))
            .flatten()
            .collect::<AHashSet<_>>()
            .into_iter()
            .filter_map(|id| self.units.get(id))
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

        let src_unit = Unit::create(vec3a(0.0, 0.0, 0.0), UnitKind::Player);
        service.add_unit(src_unit.clone());

        let unit = service.get_unit(src_unit.id).unwrap();
        assert_eq!(unit.id, src_unit.id);
        assert_eq!(unit.position, src_unit.position);
        assert_eq!(unit.kind, src_unit.kind);
    }

    #[test]
    fn remove_unit() {
        let mut service = UnitService::default();

        let unit = Unit::create(vec3a(0.0, 0.0, 0.0), UnitKind::Player);
        service.add_unit(unit.clone());
        service.remove_unit(unit.id);

        let is_none = service.get_unit(unit.id).is_none();
        assert!(is_none);
    }

    #[test]
    fn set_aabb_before_fill_data() {
        let mut service = UnitService::default();

        let src_unit = Unit::create(vec3a(0.0, 0.0, 0.0), UnitKind::Player);
        service.add_unit(src_unit.clone());

        let other_unit = Unit::create(vec3a(-4.0, -4.0, -4.0), UnitKind::Player);
        service.add_unit(other_unit);

        let units = service.get_units(aabb3a(vec3a(0.0, 0.0, 0.0), vec3a(8.0, 8.0, 8.0)));
        assert_eq!(units.len(), 1);

        let unit = units.get(0).unwrap();
        assert_eq!(unit.id, src_unit.id);
        assert_eq!(unit.position, src_unit.position);
        assert_eq!(unit.kind, src_unit.kind);
    }
}
