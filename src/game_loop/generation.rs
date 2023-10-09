//! ワールド生成の機能に関するモジュール

use aabb::*;
use ahash::HashSet;
use glam::*;

use crate::{assets, game_loop::block};

/// ワールド生成の機能
pub struct GenerationSystem {
    init_flags: HashSet<IVec2>,
}

impl GenerationSystem {
    /// 空間分割サイズ
    const GRID_SIZE: i32 = 32;

    #[inline]
    pub fn new() -> Self {
        Self {
            init_flags: Default::default(),
        }
    }

    /// 指定した範囲のワールドを生成する。
    pub fn generate(
        &mut self,
        assets: &assets::Assets,
        block_system: &mut block::BlockSystem,
        bounds: IAabb2,
    ) {
        fn iter_point(bounds: IAabb2) -> impl Iterator<Item = IVec2> {
            let min = bounds.min;
            let max = bounds.max;
            (min.x..=max.x).flat_map(move |x| (min.y..=max.y).map(move |y| ivec2(x, y)))
        }

        let bounds = bounds.div_euclid_i32(Self::GRID_SIZE);
        for grid_point in iter_point(bounds) {
            if self.init_flags.contains(&grid_point) {
                continue;
            }

            let bounds = iaabb2(grid_point, grid_point + ivec2(1, 1)) * Self::GRID_SIZE;

            for spec in assets.generation_specs() {
                match spec {
                    assets::GenerationSpec::RandomBlock {
                        block_spec_id,
                        probability,
                    } => {
                        for position in iter_point(bounds) {
                            if *probability < rand::random::<f32>() {
                                continue;
                            }

                            let block = block::Block {
                                spec_id: *block_spec_id,
                                position,
                            };
                            block_system.insert(block);
                        }
                    }
                    assets::GenerationSpec::FillBlock { block_spec_id } => {
                        for position in iter_point(bounds) {
                            let block = block::Block {
                                spec_id: *block_spec_id,
                                position,
                            };
                            block_system.insert(block);
                        }
                    }
                }
            }

            self.init_flags.insert(grid_point);
        }
    }
}
