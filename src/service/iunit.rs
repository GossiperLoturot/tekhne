use crate::model::*;
use ahash::{AHashMap, AHashSet};
use glam::*;
use slab::Slab;

#[derive(Default)]
pub struct IUnitService {
    iunits: Slab<IUnit>,
    index_table: AHashMap<IVec3, AHashSet<usize>>,
}

impl IUnitService {
    const GRID_SIZE: f32 = 32.0;

    pub fn insert(&mut self, iunit: IUnit) -> usize {
        let id = self.iunits.vacant_key();

        let grid_aabb = (iunit.aabb().as_aabb3a() / Self::GRID_SIZE)
            .floor()
            .as_iaabb3();
        grid_aabb.iter().for_each(|index| {
            self.index_table.entry(index).or_default().insert(id);
        });

        self.iunits.insert(iunit)
    }

    pub fn remove(&mut self, id: usize) -> Option<IUnit> {
        let iunit = self.iunits.try_remove(id);

        if let Some(iunit) = iunit {
            let grid_aabb = (iunit.aabb().as_aabb3a() / Self::GRID_SIZE)
                .floor()
                .as_iaabb3();
            grid_aabb.iter().for_each(|index| {
                self.index_table.entry(index).or_default().remove(&id);
            });

            Some(iunit)
        } else {
            None
        }
    }

    pub fn get(&self, id: usize) -> Option<&IUnit> {
        self.iunits.get(id)
    }

    pub fn get_by_aabb(&self, aabb: IAabb3) -> Vec<(usize, &IUnit)> {
        let grid_aabb = (aabb.as_aabb3a() / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb
            .iter()
            .filter_map(|index| self.index_table.get(&index))
            .flatten()
            .collect::<AHashSet<_>>()
            .into_iter()
            .filter_map(|&id| self.iunits.get(id).map(|iunit| (id, iunit)))
            .filter(|(_, iunit)| aabb.intersect(iunit.aabb()))
            .collect::<Vec<_>>()
    }
}
