//! ワールド生成の機能に関するモジュール

use aabb::*;
use ahash::HashSet;
use glam::*;
use itertools::Itertools;

use crate::{
    assets,
    game_loop::{base, block, entity},
};

/// ワールド生成の機能
pub struct GenerationSystem {
    grid_flags: HashSet<IVec2>,
}

impl GenerationSystem {
    /// 空間分割サイズ
    const GRID_SIZE: i32 = 32;

    /// 範囲の外側に余剰に生成するグリッドの大きさ
    const EXTEND_GRID: i32 = 2;

    #[inline]
    pub fn new() -> Self {
        Self {
            grid_flags: Default::default(),
        }
    }

    /// 指定した範囲のワールドを生成する。
    pub fn update(
        &mut self,
        assets: &assets::Assets,
        base_system: &mut base::BaseSystem,
        block_system: &mut block::BlockSystem,
        entity_system: &mut entity::EntitySystem,
        bounds: Aabb2,
    ) {
        let grid_bounds = bounds
            .trunc_over()
            .as_iaabb2()
            .to_grid_space(Self::GRID_SIZE)
            .extends(Self::EXTEND_GRID);

        grid_bounds
            .into_iter_points()
            .filter(|grid_point| !self.grid_flags.contains(grid_point))
            .cartesian_product(&assets.generation_specs)
            .for_each(|(grid_point, generation_spec)| match generation_spec {
                assets::GenerationSpec::FillBase { base_spec_id, .. } => {
                    grid_point
                        .to_base_space(Self::GRID_SIZE)
                        .into_iter_points()
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
                    grid_point
                        .to_base_space(Self::GRID_SIZE)
                        .into_iter_points()
                        .filter(|_| rand::random::<f32>() < *probability)
                        .for_each(|position| {
                            let z_random = rand::random();
                            let block = block::Block::new(*block_spec_id, position, z_random);
                            block_system.insert(assets, block);
                        });
                }
            });

        grid_bounds.into_iter_points().for_each(|grid_point| {
            self.grid_flags.insert(grid_point);
        });
    }
}
