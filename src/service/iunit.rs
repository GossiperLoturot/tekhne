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

    pub fn add_iunit(&mut self, iunit: IUnit) -> usize {
        let id = self.iunits.vacant_key();

        let grid_aabb = (iunit.aabb().as_aabb3a() / Self::GRID_SIZE)
            .floor()
            .as_iaabb3();
        grid_aabb.iter().for_each(|index| {
            self.index_table.entry(index).or_default().insert(id);
        });

        self.iunits.insert(iunit)
    }

    pub fn remove_iunit(&mut self, id: usize) -> Option<IUnit> {
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

    pub fn get_iunit(&self, id: usize) -> Option<&IUnit> {
        self.iunits.get(id)
    }

    pub fn get_iunits(&self, aabb: IAabb3) -> Vec<&IUnit> {
        let grid_aabb = (aabb.as_aabb3a() / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb
            .iter()
            .filter_map(|grid_index| self.index_table.get(&grid_index))
            .flatten()
            .collect::<AHashSet<_>>()
            .into_iter()
            .filter_map(|id| self.iunits.get(*id))
            .filter(|iunit| aabb.contains(iunit.position))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_iunit() {
        let mut service = IUnitService::default();

        let id = service.add_iunit(IUnit::new(ivec3(0, 0, 0), IUnitKind::SurfaceDirt));

        let iunit = service.get_iunit(id).unwrap();
        assert_eq!(iunit.position, ivec3(0, 0, 0));
        assert_eq!(iunit.kind, IUnitKind::SurfaceDirt);
    }

    #[test]
    fn remove_iunit() {
        let mut service = IUnitService::default();

        let id = service.add_iunit(IUnit::new(ivec3(0, 0, 0), IUnitKind::SurfaceDirt));
        service.remove_iunit(id);

        assert!(service.get_iunit(id).is_none());
    }

    #[test]
    fn set_aabb_before_fill_data() {
        let mut service = IUnitService::default();

        service.add_iunit(IUnit::new(ivec3(0, 0, 0), IUnitKind::SurfaceDirt));
        service.add_iunit(IUnit::new(ivec3(-1, -1, -1), IUnitKind::SurfaceGrass));

        let iunits = service.get_iunits(iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8)));
        assert_eq!(iunits.len(), 1);

        let iunit = iunits.get(0).unwrap();
        assert_eq!(iunit.position, ivec3(0, 0, 0));
        assert_eq!(iunit.kind, IUnitKind::SurfaceDirt);
    }
}
