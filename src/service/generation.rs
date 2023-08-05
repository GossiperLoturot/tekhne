use super::{IUnitService, UnitService};
use crate::model::*;
use ahash::AHashSet;
use glam::*;

#[derive(Default)]
pub struct GenerationService {
    init_flags: AHashSet<IVec2>,
}

impl GenerationService {
    pub fn generate(
        &mut self,
        aabb: IAabb3,
        iunit_service: &mut IUnitService,
        unit_service: &mut UnitService,
    ) {
        for x in aabb.min.x..=aabb.max.x {
            for y in aabb.min.y..=aabb.max.y {
                let pos = IVec2::new(x, y);
                if !self.init_flags.contains(&pos) {
                    // generation rules start

                    iunit_service
                        .add_iunit(IUnit::new(IVec3::new(x, y, 0), UnitKind::SurfaceGrass));

                    if rand::random::<f32>() < 0.08 {
                        iunit_service
                            .add_iunit(IUnit::new(IVec3::new(x, y, 1), UnitKind::MixGrass));
                    }

                    if rand::random::<f32>() < 0.02 {
                        iunit_service
                            .add_iunit(IUnit::new(IVec3::new(x, y, 1), UnitKind::Dandelion));
                    }

                    if rand::random::<f32>() < 0.01 {
                        iunit_service
                            .add_iunit(IUnit::new(IVec3::new(x, y, 1), UnitKind::FallenLeaves));
                    }

                    if rand::random::<f32>() < 0.01 {
                        iunit_service
                            .add_iunit(IUnit::new(IVec3::new(x, y, 1), UnitKind::FallenBranch));
                    }

                    if rand::random::<f32>() < 0.04 {
                        iunit_service
                            .add_iunit(IUnit::new(IVec3::new(x, y, 1), UnitKind::MixPebbles));
                    }

                    if rand::random::<f32>() < 0.02 {
                        iunit_service.add_iunit(IUnit::new(IVec3::new(x, y, 1), UnitKind::OakTree));
                    }

                    if rand::random::<f32>() < 0.02 {
                        iunit_service
                            .add_iunit(IUnit::new(IVec3::new(x, y, 1), UnitKind::BirchTree));
                    }

                    if rand::random::<f32>() < 0.001 {
                        iunit_service
                            .add_iunit(IUnit::new(IVec3::new(x, y, 1), UnitKind::DyingTree));
                    }

                    if rand::random::<f32>() < 0.001 {
                        iunit_service
                            .add_iunit(IUnit::new(IVec3::new(x, y, 1), UnitKind::FallenTree));
                    }

                    if rand::random::<f32>() < 0.01 {
                        iunit_service.add_iunit(IUnit::new(IVec3::new(x, y, 1), UnitKind::MixRock));
                    }

                    // generation rules end

                    self.init_flags.insert(pos);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let mut iunit_service = IUnitService::default();
        let mut unit_service = UnitService::default();
        let mut gen_service = GenerationService::default();

        let aabb = IAabb3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        gen_service.generate(aabb, &mut iunit_service, &mut unit_service);
    }
}
