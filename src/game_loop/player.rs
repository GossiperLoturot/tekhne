//! プレイヤーシステムの機能に関するモジュール

use glam::*;

use crate::{assets, game_loop::entity};

/// プレイヤーシステムの機能
pub struct PlayerSystem {
    player_id: Option<usize>,
}

impl PlayerSystem {
    /// 通常時の移動速度
    const DEFAULT_SPEED: f32 = 2.0;

    /// スプリント時の移動速度
    const SPRINT_SPEED: f32 = 4.0;

    #[inline]
    pub fn new() -> Self {
        Self {
            player_id: Default::default(),
        }
    }

    /// 新しいプレイヤーを作成し、そのエンティティの参照を返す。
    #[inline]
    pub fn spawn_player<'a>(
        &mut self,
        assets: &'a assets::Assets,
        entity_system: &'a mut entity::EntitySystem,
    ) -> Option<&'a entity::Entity> {
        if self.player_id.is_none() {
            // TODO: customizable player entity picking
            let entity = entity::Entity::new(0, vec2(0.0, 0.0));
            self.player_id = entity_system.insert(assets, entity);
            self.get_player(entity_system)
        } else {
            None
        }
    }

    /// プレイヤーを削除し、そのエンティティを返す。
    #[inline]
    pub fn despawn_player(
        &mut self,
        assets: &assets::Assets,
        entity_system: &mut entity::EntitySystem,
    ) -> Option<entity::Entity> {
        self.player_id
            .and_then(|id| entity_system.remove(assets, id))
    }

    /// プレイヤーの参照を返す。
    #[inline]
    pub fn get_player<'a>(
        &self,
        entity_system: &'a entity::EntitySystem,
    ) -> Option<&'a entity::Entity> {
        self.player_id.and_then(|id| entity_system.get(id))
    }

    /// ゲームサイクルにおけるプレイヤーの振る舞いを実行する。
    ///
    /// 振る舞いは以下のとおりである。
    ///
    /// - プレイヤーはLShiftを押下時スプリントを行う。
    /// - プレイヤーはWSADで上下左右の移動を行う。
    pub fn update(
        &mut self,
        assets: &assets::Assets,
        input: &winit_input_helper::WinitInputHelper,
        entity_system: &mut entity::EntitySystem,
        elased: std::time::Duration,
    ) {
        if let Some(id) = self.player_id {
            if let Some(mut player) = entity_system.remove(assets, id) {
                let speed = if input.key_held(winit::keyboard::KeyCode::ShiftLeft) {
                    Self::SPRINT_SPEED
                } else {
                    Self::DEFAULT_SPEED
                };

                if input.key_held(winit::keyboard::KeyCode::KeyW) {
                    player.position.y += speed * elased.as_secs_f32();
                }
                if input.key_held(winit::keyboard::KeyCode::KeyS) {
                    player.position.y -= speed * elased.as_secs_f32();
                }
                if input.key_held(winit::keyboard::KeyCode::KeyA) {
                    player.position.x -= speed * elased.as_secs_f32();
                }
                if input.key_held(winit::keyboard::KeyCode::KeyD) {
                    player.position.x += speed * elased.as_secs_f32();
                }

                entity_system.insert(assets, player);
            }
        }
    }
}
