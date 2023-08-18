//! ワールド生成システムに関するモジュール

use super::{BlockSystem, EntitySystem};
use crate::model::*;
use ahash::AHashSet;
use glam::*;

/// ワールド生成の操作を行うシステム
#[derive(Default)]
pub struct GenerationSystem {
    init_flags: AHashSet<IVec2>,
}

impl GenerationSystem {
    /// ワールドの生成を実行する。
    ///
    /// 範囲`aabb`内での生成を実行し、生成されたデータは`block_system`、`entity_system`に保存される。
    pub fn generate(
        &mut self,
        aabb: IAabb3,
        block_system: &mut BlockSystem,
        entity_system: &mut EntitySystem,
    ) {
        for x in aabb.min.x..=aabb.max.x {
            for y in aabb.min.y..=aabb.max.y {
                let pos = ivec2(x, y);
                if !self.init_flags.contains(&pos) {
                    // generation rules start

                    block_system.insert(Block::new(ivec3(x, y, 0), BlockKind::SurfaceGrass));

                    if rand::random::<f32>() < 0.08 {
                        block_system.insert(Block::new(ivec3(x, y, 1), BlockKind::MixGrass));
                    }

                    if rand::random::<f32>() < 0.02 {
                        block_system.insert(Block::new(ivec3(x, y, 1), BlockKind::Dandelion));
                    }

                    if rand::random::<f32>() < 0.01 {
                        block_system.insert(Block::new(ivec3(x, y, 1), BlockKind::FallenLeaves));
                    }

                    if rand::random::<f32>() < 0.01 {
                        block_system.insert(Block::new(ivec3(x, y, 1), BlockKind::FallenBranch));
                    }

                    if rand::random::<f32>() < 0.04 {
                        block_system.insert(Block::new(ivec3(x, y, 1), BlockKind::MixPebbles));
                    }

                    if rand::random::<f32>() < 0.02 {
                        block_system.insert(Block::new(ivec3(x, y, 1), BlockKind::OakTree));
                    }

                    if rand::random::<f32>() < 0.02 {
                        block_system.insert(Block::new(ivec3(x, y, 1), BlockKind::BirchTree));
                    }

                    if rand::random::<f32>() < 0.001 {
                        block_system.insert(Block::new(ivec3(x, y, 1), BlockKind::DyingTree));
                    }

                    if rand::random::<f32>() < 0.001 {
                        block_system.insert(Block::new(ivec3(x, y, 1), BlockKind::FallenTree));
                    }

                    if rand::random::<f32>() < 0.01 {
                        block_system.insert(Block::new(ivec3(x, y, 1), BlockKind::MixRock));
                    }

                    // generation rules end

                    self.init_flags.insert(pos);
                }
            }
        }
    }
}
