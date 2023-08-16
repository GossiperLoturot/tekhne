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

    pub fn insert(&mut self, unit: Unit) -> usize {
        let id = self.units.vacant_key();

        let grid_aabb = (unit.aabb() / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb.iter().for_each(|index| {
            self.index_table.entry(index).or_default().insert(id);
        });

        self.units.insert(unit)
    }

    pub fn remove(&mut self, id: usize) -> Option<Unit> {
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

    pub fn get(&self, id: usize) -> Option<&Unit> {
        self.units.get(id)
    }

    pub fn get_by_aabb(&self, aabb: Aabb3A) -> Vec<(usize, &Unit)> {
        let grid_aabb = (aabb / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb
            .iter()
            .filter_map(|index| self.index_table.get(&index))
            .flatten()
            .collect::<AHashSet<_>>()
            .into_iter()
            .filter_map(|&id| self.units.get(id).map(|unit| (id, unit)))
            .filter(|(_, unit)| aabb.intersect(unit.aabb()))
            .collect::<Vec<_>>()
    }
}
