use crate::model::*;
use ahash::{AHashMap, AHashSet};
use glam::*;

#[derive(Default)]
pub struct IUnitService {
    iunits: AHashMap<IVec3, IUnit>,
    index_table: AHashMap<IVec3, AHashSet<IVec3>>,
}

impl IUnitService {
    const GRID_SIZE: f32 = 32.0;

    pub fn add_iunit(&mut self, iunit: IUnit) -> Option<IUnit> {
        if !self.iunits.contains_key(&iunit.position) {
            let grid_index = (iunit.position.as_vec3a() / Self::GRID_SIZE)
                .floor()
                .as_ivec3();
            self.index_table
                .entry(grid_index)
                .or_default()
                .insert(iunit.position);

            self.iunits.insert(iunit.position, iunit)
        } else {
            None
        }
    }

    pub fn remove_iunit(&mut self, position: IVec3) -> Option<IUnit> {
        if let Some(iunit) = self.iunits.get(&position) {
            let grid_index = (iunit.position.as_vec3a() / Self::GRID_SIZE)
                .floor()
                .as_ivec3();
            self.index_table
                .entry(grid_index)
                .or_default()
                .remove(&iunit.position);

            self.iunits.remove(&position)
        } else {
            None
        }
    }

    pub fn get_iunit(&self, pos: IVec3) -> Option<&IUnit> {
        self.iunits.get(&pos)
    }

    pub fn get_iunits(&self, aabb: IAabb3) -> Vec<&IUnit> {
        let grid_aabb = (aabb.as_aabb3a() / Self::GRID_SIZE).floor().as_iaabb3();
        grid_aabb
            .iter()
            .filter_map(|grid_index| self.index_table.get(&grid_index))
            .flatten()
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
        service.add_iunit(IUnit::new(ivec3(0, 0, 0), IUnitKind::SurfaceDirt));

        let iunit = service.get_iunit(ivec3(0, 0, 0)).unwrap();
        assert_eq!(iunit.position, ivec3(0, 0, 0));
        assert_eq!(iunit.kind, IUnitKind::SurfaceDirt);
    }

    #[test]
    fn remove_iunit() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(ivec3(0, 0, 0), IUnitKind::SurfaceDirt));
        service.remove_iunit(ivec3(0, 0, 0));

        let is_none = service.get_iunit(ivec3(0, 0, 0)).is_none();
        assert!(is_none);
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
