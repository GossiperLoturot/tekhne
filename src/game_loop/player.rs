//! プレイヤーシステムの機能に関するモジュール

use glam::*;

use crate::aabb::*;
use crate::{
    assets,
    game_loop::{base, block, camera, entity},
};

pub struct PlayerState {
    pub entity_id: usize,
    pub target_base: Option<usize>,
    pub target_block: Option<usize>,
    pub target_entity: Option<usize>,
}

/// プレイヤーシステムの機能
pub struct PlayerSystem {
    assets: std::rc::Rc<assets::Assets>,
    player_state: Option<PlayerState>,
}

impl PlayerSystem {
    /// 通常時の移動速度
    const DEFAULT_SPEED: f32 = 2.0;

    /// スプリント時の移動速度
    const SPRINT_SPEED: f32 = 4.0;

    /// 新しいプレイヤーシステムを作成する。
    #[inline]
    pub fn new(assets: std::rc::Rc<assets::Assets>) -> Self {
        Self {
            assets,
            player_state: Default::default(),
        }
    }

    /// ゲームサイクルにおける振る舞いを実行する。
    pub fn update(
        &mut self,
        input: &winit_input_helper::WinitInputHelper,
        tick: &std::time::Duration,
        window_size: (u32, u32),
        base_storage: &mut base::BaseStorage,
        block_storage: &mut block::BlockStorage,
        entity_storage: &mut entity::EntityStorage,
        camera_sys: &camera::CameraSystem,
    ) {
        match &mut self.player_state {
            None => {
                // NOTE: プレイヤーをワールド上に作成する。
                let player_spec = &self.assets.player_specs[0];
                let entity_spec = &self.assets.entity_specs[player_spec.entity_spec_id];

                // NOTE: シームレスな焦点位置
                let position = Vec2::ZERO - entity_spec.rendering_size.center();

                let entity = entity::Entity::new(entity_spec.id, position);
                let entity_id = entity_storage.insert(entity).unwrap();

                self.player_state = Some(PlayerState {
                    entity_id,
                    target_base: None,
                    target_block: None,
                    target_entity: None,
                });
            }
            Some(player) => {
                let entity = entity_storage.remove(player.entity_id).unwrap();

                // NOTE: スプリント or 通常
                let speed = if input.key_held(winit::keyboard::KeyCode::ShiftLeft) {
                    Self::SPRINT_SPEED
                } else {
                    Self::DEFAULT_SPEED
                };

                // NOTE: プレイヤーの移動
                let mut move_entity = entity.clone();
                if input.key_held(winit::keyboard::KeyCode::KeyW) {
                    move_entity.position.y += speed * tick.as_secs_f32();
                }
                if input.key_held(winit::keyboard::KeyCode::KeyS) {
                    move_entity.position.y -= speed * tick.as_secs_f32();
                }
                if input.key_held(winit::keyboard::KeyCode::KeyA) {
                    move_entity.position.x -= speed * tick.as_secs_f32();
                }
                if input.key_held(winit::keyboard::KeyCode::KeyD) {
                    move_entity.position.x += speed * tick.as_secs_f32();
                }
                player.entity_id = entity_storage.insert(move_entity).unwrap();

                // NOTE: オブジェクトの選択
                let (x, y) = input.mouse_diff();
                let matrix = camera_sys.get().world_to_viewport(window_size).inverse();

                let position = matrix.project_point3(vec3(x, y, 0.0)).xy();
                let rect = aabb2(position, position);

                player.target_base = base_storage
                    .get_rendering_by_rect(rect)
                    .map(|(id, _)| id)
                    .next();
                player.target_block = block_storage
                    .get_rendering_by_rect(rect)
                    .map(|(id, _)| id)
                    .next();
                player.target_entity = entity_storage
                    .get_rendering_by_rect(rect)
                    .map(|(id, _)| id)
                    .next();
            }
        }
    }

    pub fn get_player(&self) -> Option<&PlayerState> {
        self.player_state.as_ref()
    }
}
