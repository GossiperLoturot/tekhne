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

                    iunit_service.insert(IUnit::new(ivec3(x, y, 0), IUnitKind::SurfaceGrass));

                    if rand::random::<f32>() < 0.08 {
                        iunit_service.insert(IUnit::new(ivec3(x, y, 1), IUnitKind::MixGrass));
                    }

                    if rand::random::<f32>() < 0.02 {
                        iunit_service.insert(IUnit::new(ivec3(x, y, 1), IUnitKind::Dandelion));
                    }

                    if rand::random::<f32>() < 0.01 {
                        iunit_service.insert(IUnit::new(ivec3(x, y, 1), IUnitKind::FallenLeaves));
                    }

                    if rand::random::<f32>() < 0.01 {
                        iunit_service.insert(IUnit::new(ivec3(x, y, 1), IUnitKind::FallenBranch));
                    }

                    if rand::random::<f32>() < 0.04 {
                        iunit_service.insert(IUnit::new(ivec3(x, y, 1), IUnitKind::MixPebbles));
                    }

                    if rand::random::<f32>() < 0.02 {
                        iunit_service.insert(IUnit::new(ivec3(x, y, 1), IUnitKind::OakTree));
                    }

                    if rand::random::<f32>() < 0.02 {
                        iunit_service.insert(IUnit::new(ivec3(x, y, 1), IUnitKind::BirchTree));
                    }

                    if rand::random::<f32>() < 0.001 {
                        iunit_service.insert(IUnit::new(ivec3(x, y, 1), IUnitKind::DyingTree));
                    }

                    if rand::random::<f32>() < 0.001 {
                        iunit_service.insert(IUnit::new(ivec3(x, y, 1), IUnitKind::FallenTree));
                    }

                    if rand::random::<f32>() < 0.01 {
                        iunit_service.insert(IUnit::new(ivec3(x, y, 1), IUnitKind::MixRock));
                    }

                    // generation rules end

                    self.init_flags.insert(pos);
                }
            }
        }
    }
}
