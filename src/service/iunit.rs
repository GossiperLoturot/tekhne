use crate::model::*;
use ahash::{AHashMap, AHashSet};
use glam::*;

#[derive(Default)]
pub struct IUnitService {
    iunits: AHashMap<u64, IUnit>,
    index_table: AHashMap<IVec3, AHashSet<u64>>,
}

impl IUnitService {
    const GRID_SIZE: f32 = 32.0;

    pub fn add_iunit(&mut self, iunit: IUnit) -> Option<&IUnit> {
        let id = iunit.id;

        if !self.iunits.contains_key(&id) {
            let grid_aabb = (iunit.aabb().as_aabb3a() / Self::GRID_SIZE)
                .floor()
                .as_iaabb3();
            grid_aabb.iter().for_each(|index| {
                self.index_table.entry(index).or_default().insert(id);
            });

            self.iunits.insert(id, iunit);
            self.iunits.get(&id)
        } else {
            None
        }
    }

    pub fn remove_iunit(&mut self, id: u64) -> Option<IUnit> {
        if let Some(iunit) = self.iunits.get(&id) {
            let grid_aabb = (iunit.aabb().as_aabb3a() / Self::GRID_SIZE)
                .floor()
                .as_iaabb3();
            grid_aabb.iter().for_each(|index| {
                self.index_table.entry(index).or_default().remove(&id);
            });

            self.iunits.remove(&id)
        } else {
            None
        }
    }

    pub fn get_iunit(&self, id: u64) -> Option<&IUnit> {
        self.iunits.get(&id)
    }

    pub fn get_iunits(&self, aabb: IAabb3) -> Vec<&IUnit> {
        let grid_aabb = (aabb.as_aabb3a() / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb
            .iter()
            .filter_map(|grid_index| self.index_table.get(&grid_index))
            .flatten()
            .collect::<AHashSet<_>>()
            .into_iter()
            .filter_map(|position| self.iunits.get(position))
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

        let src_iunit = IUnit::create(ivec3(0, 0, 0), IUnitKind::SurfaceDirt);
        service.add_iunit(src_iunit.clone());

        let iunit = service.get_iunit(src_iunit.id).unwrap();
        assert_eq!(iunit.id, src_iunit.id);
        assert_eq!(iunit.position, src_iunit.position);
        assert_eq!(iunit.kind, src_iunit.kind);
    }

    #[test]
    fn remove_iunit() {
        let mut service = IUnitService::default();

        let iunit = IUnit::create(ivec3(0, 0, 0), IUnitKind::SurfaceDirt);
        service.add_iunit(iunit.clone());
        service.remove_iunit(iunit.id);

        let is_none = service.get_iunit(iunit.id).is_none();
        assert!(is_none);
    }

    #[test]
    fn set_aabb_before_fill_data() {
        let mut service = IUnitService::default();

        let src_iunit = IUnit::create(ivec3(0, 0, 0), IUnitKind::SurfaceDirt);
        service.add_iunit(src_iunit.clone());

        let other_iunit = IUnit::create(ivec3(-1, -1, -1), IUnitKind::SurfaceGrass);
        service.add_iunit(other_iunit);

        let iunits = service.get_iunits(iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8)));
        assert_eq!(iunits.len(), 1);

        let iunit = iunits.get(0).unwrap();
        assert_eq!(iunit.id, src_iunit.id);
        assert_eq!(iunit.position, src_iunit.position);
        assert_eq!(iunit.kind, src_iunit.kind);
    }
}
