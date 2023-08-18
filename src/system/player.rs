//! プレイヤーシステムに関するモジュール

use super::EntitySystem;
use crate::model::*;
use glam::*;

/// ワールド上のプレイヤーの操作を行うシステム
///
/// プレイヤーはワールド内に最大1個のみである。
#[derive(Default)]
pub struct PlayerSystem {
    player_id: Option<usize>,
}

impl PlayerSystem {
    /// 通常の移動速度
    const DEFAULT_SPEED: f32 = 2.0;

    /// スプリント時の移動速度
    const SPRINT_SPEED: f32 = 4.0;

    /// ワールド上に新しいプレイヤーを作成する。
    ///
    /// ワールド上にプレイヤーが存在せず、作成に成功した場合は`Some(&Entity)`を返す。
    /// 逆にプレイヤーが存在し、作成に失敗した場合は`None`を返す。
    pub fn spawn_player<'a>(&mut self, entity_system: &'a mut EntitySystem) -> Option<&'a Entity> {
        if self.player_id.is_none() {
            let id = entity_system.insert(Entity::new(vec3a(0.0, 0.0, 1.0), EntityKind::Player));
            self.player_id = Some(id);
            self.get_player(entity_system)
        } else {
            None
        }
    }

    /// ワールド上のプレイヤーを削除する。
    ///
    /// ワールド上にプレイヤーが存在し、削除に成功した場合は`Some(Entity)`を返す。
    /// 逆にプレイヤーが存在せず、削除に失敗した場合は`None`を返す。
    pub fn despawn_player(&mut self, entity_system: &mut EntitySystem) -> Option<Entity> {
        self.player_id.and_then(|id| entity_system.remove(id))
    }

    /// ワールド上のプレイヤーを取得する。
    ///
    /// ワールド上にプレイヤーが存在する場合は`Some(&Entity)`を返す。
    /// 逆にプレイヤーが存在しない場合は`None`を返す。
    pub fn get_player<'a>(&self, entity_system: &'a EntitySystem) -> Option<&'a Entity> {
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
        entity_system: &mut EntitySystem,
        input: &winit_input_helper::WinitInputHelper,
        elased: std::time::Duration,
    ) {
        if let Some(id) = self.player_id {
            if let Some(mut player) = entity_system.remove(id) {
                let speed = if input.key_held(winit::event::VirtualKeyCode::LShift) {
                    Self::SPRINT_SPEED
                } else {
                    Self::DEFAULT_SPEED
                };

                if input.key_held(winit::event::VirtualKeyCode::W) {
                    player.position.y += speed * elased.as_secs_f32();
                }
                if input.key_held(winit::event::VirtualKeyCode::S) {
                    player.position.y -= speed * elased.as_secs_f32();
                }
                if input.key_held(winit::event::VirtualKeyCode::A) {
                    player.position.x -= speed * elased.as_secs_f32();
                }
                if input.key_held(winit::event::VirtualKeyCode::D) {
                    player.position.x += speed * elased.as_secs_f32();
                }

                entity_system.insert(player);
            }
        }
    }
}
