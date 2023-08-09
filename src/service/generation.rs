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
                let pos = ivec2(x, y);
                if !self.init_flags.contains(&pos) {
                    // generation rules start

                    iunit_service.add_iunit(IUnit::create(ivec3(x, y, 0), IUnitKind::SurfaceGrass));

                    if rand::random::<f32>() < 0.08 {
                        iunit_service.add_iunit(IUnit::create(ivec3(x, y, 1), IUnitKind::MixGrass));
                    }

                    if rand::random::<f32>() < 0.02 {
                        iunit_service
                            .add_iunit(IUnit::create(ivec3(x, y, 1), IUnitKind::Dandelion));
                    }

                    if rand::random::<f32>() < 0.01 {
                        iunit_service
                            .add_iunit(IUnit::create(ivec3(x, y, 1), IUnitKind::FallenLeaves));
                    }

                    if rand::random::<f32>() < 0.01 {
                        iunit_service
                            .add_iunit(IUnit::create(ivec3(x, y, 1), IUnitKind::FallenBranch));
                    }

                    if rand::random::<f32>() < 0.04 {
                        iunit_service
                            .add_iunit(IUnit::create(ivec3(x, y, 1), IUnitKind::MixPebbles));
                    }

                    if rand::random::<f32>() < 0.02 {
                        iunit_service.add_iunit(IUnit::create(ivec3(x, y, 1), IUnitKind::OakTree));
                    }

                    if rand::random::<f32>() < 0.02 {
                        iunit_service
                            .add_iunit(IUnit::create(ivec3(x, y, 1), IUnitKind::BirchTree));
                    }

                    if rand::random::<f32>() < 0.001 {
                        iunit_service
                            .add_iunit(IUnit::create(ivec3(x, y, 1), IUnitKind::DyingTree));
                    }

                    if rand::random::<f32>() < 0.001 {
                        iunit_service
                            .add_iunit(IUnit::create(ivec3(x, y, 1), IUnitKind::FallenTree));
                    }

                    if rand::random::<f32>() < 0.01 {
                        iunit_service.add_iunit(IUnit::create(ivec3(x, y, 1), IUnitKind::MixRock));
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

        let aabb = iaabb3(ivec3(0, 0, 0), ivec3(8, 8, 8));
        gen_service.generate(aabb, &mut iunit_service, &mut unit_service);
    }
}
