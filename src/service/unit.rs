use crate::model::*;
use ahash::{AHashMap, AHashSet};
use glam::*;
use uuid::*;

#[derive(Default)]
pub struct UnitService {
    units: AHashMap<Uuid, Unit>,
    index_table: AHashMap<IVec3, AHashSet<Uuid>>,
}

impl UnitService {
    const GRID_SIZE: f32 = 32.0;

    pub fn add_unit(&mut self, unit: Unit) -> Option<Unit> {
        if !self.units.contains_key(&unit.id) {
            let grid_index = (unit.position / Self::GRID_SIZE).floor().as_ivec3();
            self.index_table
                .entry(grid_index)
                .or_default()
                .insert(unit.id);

            self.units.insert(unit.id, unit)
        } else {
            None
        }
    }

    pub fn remove_unit(&mut self, id: &Uuid) -> Option<Unit> {
        if let Some(unit) = self.units.get(id) {
            let grid_index = (unit.position / Self::GRID_SIZE).floor().as_ivec3();
            self.index_table
                .entry(grid_index)
                .or_default()
                .remove(&unit.id);

            self.units.remove(id)
        } else {
            None
        }
    }

    pub fn get_unit(&self, id: &Uuid) -> Option<&Unit> {
        self.units.get(id)
    }

    pub fn get_units(&self, aabb: Aabb3A) -> Vec<&Unit> {
        let grid_aabb = (aabb / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb
            .iter()
            .filter_map(|grid_index| self.index_table.get(&grid_index))
            .flatten()
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

        let id = Uuid::new_v4();
        service.add_unit(Unit::new(id, vec3a(0.0, 0.0, 0.0), UnitKind::Player));

        let unit = service.get_unit(&id).unwrap();
        assert_eq!(unit.position, vec3a(0.0, 0.0, 0.0));
        assert_eq!(unit.kind, UnitKind::Player);
    }

    #[test]
    fn remove_unit() {
        let mut service = UnitService::default();

        let id = Uuid::new_v4();
        service.add_unit(Unit::new(id, vec3a(0.0, 0.0, 0.0), UnitKind::Player));
        service.remove_unit(&id);

        let is_none = service.get_unit(&id).is_none();
        assert!(is_none);
    }

    #[test]
    fn set_aabb_before_fill_data() {
        let mut service = UnitService::default();

        let id = Uuid::new_v4();
        service.add_unit(Unit::new(id, vec3a(0.0, 0.0, 0.0), UnitKind::Player));

        let other_id = Uuid::new_v4();
        service.add_unit(Unit::new(
            other_id,
            vec3a(-4.0, -4.0, -4.0),
            UnitKind::Player,
        ));

        let units = service.get_units(aabb3a(vec3a(0.0, 0.0, 0.0), vec3a(8.0, 8.0, 8.0)));
        assert_eq!(units.len(), 1);
        let unit = units.get(0).unwrap();
        assert_eq!(unit.id, id);
        assert_eq!(unit.position, vec3a(0.0, 0.0, 0.0));
        assert_eq!(unit.kind, UnitKind::Player);
    }
}
