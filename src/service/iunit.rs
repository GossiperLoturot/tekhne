// TODO: improve performance by grid space partitioning

use crate::model::*;
use ahash::AHashMap;
use glam::*;

#[derive(Debug, Default)]
pub struct IUnitService {
    iunits: AHashMap<IVec3, IUnit>,
}

impl IUnitService {
    pub fn add_iunit(&mut self, iunit: IUnit) -> Option<()> {
        if self.iunits.contains_key(&iunit.position) {
            return None;
        }

        self.iunits.insert(iunit.position, iunit);
        Some(())
    }

    pub fn remove_iunit(&mut self, pos: IVec3) -> Option<()> {
        if !self.iunits.contains_key(&pos) {
            return None;
        }

        self.iunits.remove(&pos);
        Some(())
    }

    pub fn get_iunit(&self, pos: IVec3) -> Option<&IUnit> {
        self.iunits.get(&pos)
    }

    pub fn get_iunits(&self, bounds: Bounds<IVec3>) -> Vec<&IUnit> {
        let mut iunits = vec![];

        for x in bounds.min.x..=bounds.max.x {
            for y in bounds.min.y..=bounds.max.y {
                for z in bounds.min.z..=bounds.max.z {
                    if let Some(iunit) = self.iunits.get(&IVec3::new(x, y, z)) {
                        iunits.push(iunit);
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
        service.add_iunit(IUnit::new(IVec3::new(0, 0, 0), ResourceKind::SurfaceDirt));

        let iunit = service.get_iunit(IVec3::new(0, 0, 0)).unwrap();
        assert_eq!(iunit.position, IVec3::new(0, 0, 0));
        assert_eq!(iunit.resource_kind, ResourceKind::SurfaceDirt);
    }

    #[test]
    fn remove_iunit() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(IVec3::new(0, 0, 0), ResourceKind::SurfaceDirt));
        service.remove_iunit(IVec3::new(0, 0, 0));

        let is_none = service.get_iunit(IVec3::new(0, 0, 0)).is_none();
        assert!(is_none);
    }

    #[test]
    fn set_bounds_before_fill_data() {
        let mut service = IUnitService::default();
        service.add_iunit(IUnit::new(IVec3::new(0, 0, 0), ResourceKind::SurfaceDirt));
        service.add_iunit(IUnit::new(
            IVec3::new(-1, -1, -1),
            ResourceKind::SurfaceGrass,
        ));

        let iunits = service.get_iunits(Bounds::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8)));
        assert_eq!(iunits.len(), 1);
        let iunit = iunits.get(0).unwrap();
        assert_eq!(iunit.position, IVec3::new(0, 0, 0));
        assert_eq!(iunit.resource_kind, ResourceKind::SurfaceDirt);
    }
}
