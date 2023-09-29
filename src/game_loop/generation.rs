//! ワールド生成の機能に関するモジュール

use ahash::AHashSet;
use glam::*;

use crate::game_loop::entity;

/// ワールド生成の機能
#[derive(Default)]
pub struct GenerationSystem {
    init_flags: AHashSet<IVec2>,
}

impl GenerationSystem {
    /// 指定した範囲のワールドを生成する。
    pub fn generate(&mut self, start: Vec2, end: Vec2, entity_system: &mut entity::EntitySystem) {
        let start = start.floor().as_ivec2();
        let end = end.floor().as_ivec2();

        for x in start.x..=end.x {
            for y in start.y..=end.y {
                let pos = ivec2(x, y);
                if !self.init_flags.contains(&pos) {
                    let x = x as f32;
                    let y = y as f32;

                    if rand::random::<f32>() < 0.08 {
                        entity_system.insert(entity::Entity::new(
                            vec2(x, y),
                            entity::EntityKind::MixGrass,
                        ));
                    }

                    if rand::random::<f32>() < 0.02 {
                        entity_system.insert(entity::Entity::new(
                            vec2(x, y),
                            entity::EntityKind::Dandelion,
                        ));
                    }

                    if rand::random::<f32>() < 0.01 {
                        entity_system.insert(entity::Entity::new(
                            vec2(x, y),
                            entity::EntityKind::FallenLeaves,
                        ));
                    }

                    if rand::random::<f32>() < 0.01 {
                        entity_system.insert(entity::Entity::new(
                            vec2(x, y),
                            entity::EntityKind::FallenBranch,
                        ));
                    }

                    if rand::random::<f32>() < 0.04 {
                        entity_system.insert(entity::Entity::new(
                            vec2(x, y),
                            entity::EntityKind::MixPebbles,
                        ));
                    }

                    if rand::random::<f32>() < 0.02 {
                        entity_system
                            .insert(entity::Entity::new(vec2(x, y), entity::EntityKind::OakTree));
                    }

                    if rand::random::<f32>() < 0.02 {
                        entity_system.insert(entity::Entity::new(
                            vec2(x, y),
                            entity::EntityKind::BirchTree,
                        ));
                    }

                    if rand::random::<f32>() < 0.001 {
                        entity_system.insert(entity::Entity::new(
                            vec2(x, y),
                            entity::EntityKind::DyingTree,
                        ));
                    }

                    if rand::random::<f32>() < 0.001 {
                        entity_system.insert(entity::Entity::new(
                            vec2(x, y),
                            entity::EntityKind::FallenTree,
                        ));
                    }

                    if rand::random::<f32>() < 0.01 {
                        entity_system
                            .insert(entity::Entity::new(vec2(x, y), entity::EntityKind::MixRock));
                    }

                    self.init_flags.insert(pos);
                }
            }
        }
    }
}
