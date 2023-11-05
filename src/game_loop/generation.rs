//! ワールド生成の機能に関するモジュール

use aabb::*;
use ahash::HashSet;
use glam::*;
use itertools::Itertools;

use crate::{
    assets,
    game_loop::{base, block, camera, entity},
};

/// ワールド生成の機能
pub struct GenerationSystem {
    grid_flags: HashSet<IVec2>,
}

impl GenerationSystem {
    /// 空間分割サイズ
    const GRID_SIZE: i32 = 32;

    #[inline]
    pub fn new() -> Self {
        Self {
            grid_flags: Default::default(),
        }
    }

    /// 指定した範囲のワールドを生成する。
    pub fn generate(
        &mut self,
        assets: &assets::Assets,
        base_system: &mut base::BaseSystem,
        block_system: &mut block::BlockSystem,
        entity_system: &mut entity::EntitySystem,
        camera_system: &camera::CameraSystem,
    ) {
        if let Some(camera) = camera_system.get_camera() {
            let bounds = camera.view_bounds();
            let bounds = aabb2(bounds.min.floor(), bounds.max.ceil()).as_iaabb2();

            let bounds =
                iaabb2(bounds.min, bounds.max - IVec2::ONE).div_euclid_i32(Self::GRID_SIZE);
            let (min, max) = (bounds.min - IVec2::ONE, bounds.max + IVec2::ONE);

            itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
                .map(|(x, y)| ivec2(x, y))
                .filter(|grid_point| !self.grid_flags.contains(grid_point))
                .cartesian_product(&assets.generation_specs)
                .for_each(|(grid_point, generation_spec)| match generation_spec {
                    assets::GenerationSpec::FillBase { base_spec_id, .. } => {
                        let min = grid_point * Self::GRID_SIZE;
                        let max = grid_point * Self::GRID_SIZE + Self::GRID_SIZE;
                        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
                            .map(|(x, y)| ivec2(x, y))
                            .for_each(|position| {
                                let base = base::Base::new(*base_spec_id, position);
                                base_system.insert(base);
                            });
                    }
                    assets::GenerationSpec::RandomBlock {
                        block_spec_id,
                        probability,
                        ..
                    } => {
                        let min = grid_point * Self::GRID_SIZE;
                        let max = grid_point * Self::GRID_SIZE + Self::GRID_SIZE;
                        itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
                            .filter(|_| rand::random::<f32>() < *probability)
                            .map(|(x, y)| ivec2(x, y))
                            .for_each(|position| {
                                let block =
                                    block::Block::new(*block_spec_id, position, rand::random());
                                block_system.insert(assets, block);
                            });
                    }
                });

            itertools::Itertools::cartesian_product(min.x..max.x, min.y..max.y)
                .map(|(x, y)| ivec2(x, y))
                .for_each(|grid_point| {
                    self.grid_flags.insert(grid_point);
                });
        }
    }
}
