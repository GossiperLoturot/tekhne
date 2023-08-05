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
        let mut iunits = vec![];

        let grid_aabb = aabb.as_aabb3a().grid_partition(Self::GRID_SIZE);
        for x in grid_aabb.min.x..=grid_aabb.max.x {
            for y in grid_aabb.min.y..=grid_aabb.max.y {
                for z in grid_aabb.min.z..=grid_aabb.max.z {
                    let grid_index = IVec3::new(x, y, z);
                    if let Some(positions) = self.index_table.get(&grid_index) {
                        positions
                            .into_iter()
                            .filter_map(|position| self.iunits.get(position))
                            .filter(|iunit| aabb.contains(&iunit.position))
                            .for_each(|iunit| iunits.push(iunit));
                    }
                }
            }
        }

        iunits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_iunit() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(IVec3::new(0, 0, 0), UnitKind::SurfaceDirt));

        let iunit = service.get_iunit(IVec3::new(0, 0, 0)).unwrap();
        assert_eq!(iunit.position, IVec3::new(0, 0, 0));
        assert_eq!(iunit.kind, UnitKind::SurfaceDirt);
    }

    #[test]
    fn remove_iunit() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(IVec3::new(0, 0, 0), UnitKind::SurfaceDirt));
        service.remove_iunit(IVec3::new(0, 0, 0));

        let is_none = service.get_iunit(IVec3::new(0, 0, 0)).is_none();
        assert!(is_none);
    }

    #[test]
    fn set_aabb_before_fill_data() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(IVec3::new(0, 0, 0), UnitKind::SurfaceDirt));
        service.add_iunit(IUnit::new(IVec3::new(-1, -1, -1), UnitKind::SurfaceGrass));

        let iunits = service.get_iunits(IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8)));
        assert_eq!(iunits.len(), 1);
        let iunit = iunits.get(0).unwrap();
        assert_eq!(iunit.position, IVec3::new(0, 0, 0));
        assert_eq!(iunit.kind, UnitKind::SurfaceDirt);
    }
}
