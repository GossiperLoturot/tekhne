//! プレイヤーシステムの機能に関するモジュール

use aabb::*;
use glam::*;

use crate::game_loop::{self, base, block, camera, entity};

/// 選択しているオブジェクト
pub enum Target {
    None,
    Base(usize),
    Block(usize),
    Entity(usize),
}

/// プレイヤーが存在しない場合に保持するデータ
pub struct NoPlayer {
    pub player_spec_id: usize,
}

impl NoPlayer {
    #[inline]
    pub fn new(player_spec_id: usize) -> Self {
        Self { player_spec_id }
    }
}

/// プレイヤーデータ
pub struct Player {
    pub entity_id: usize,
    pub target: Target,
}

impl Player {
    #[inline]
    pub fn new(entity_id: usize, target: Target) -> Self {
        Self { entity_id, target }
    }
}

/// プレイヤーシステムの機能
pub enum PlayerSystem {
    NotPresent(NoPlayer),
    Present(Player),
}

impl PlayerSystem {
    /// 通常時の移動速度
    const DEFAULT_SPEED: f32 = 2.0;

    /// スプリント時の移動速度
    const SPRINT_SPEED: f32 = 4.0;

    /// 新しいプレイヤーシステムを作成する。
    #[inline]
    pub fn new() -> Self {
        Self::NotPresent(NoPlayer::new(0))
    }

    /// ゲームサイクルにおける振る舞いを実行する。
    pub fn update(
        &mut self,
        cx: &game_loop::Context,
        base_storage: &mut base::BaseStorage,
        block_storage: &mut block::BlockStorage,
        entity_storage: &mut entity::EntityStorage,
        camera_sys: &camera::CameraSystem,
    ) {
        match self {
            PlayerSystem::NotPresent(no_player) => {
                egui::Window::new("Spawn").show(cx.gui_cx, |ui| {
                    ui.label("Hello, world!");
                    if ui.button("Spawn!").clicked() {
                        println!("Spawn!");
                    }
                });

                if cx.input.key_pressed(winit::keyboard::KeyCode::KeyW) {
                    no_player.player_spec_id = no_player
                        .player_spec_id
                        .saturating_sub(1)
                        .clamp(0, cx.assets.player_specs.len() - 1);
                }

                if cx.input.key_pressed(winit::keyboard::KeyCode::KeyS) {
                    no_player.player_spec_id = no_player
                        .player_spec_id
                        .saturating_add(1)
                        .clamp(0, cx.assets.player_specs.len() - 1);
                }

                // NOTE: プレイヤーをワールド上に作成する。
                if cx.input.key_pressed(winit::keyboard::KeyCode::Enter) {
                    let player_spec = &cx.assets.player_specs[no_player.player_spec_id];
                    let entity_spec = &cx.assets.entity_specs[player_spec.entity_spec_id];

                    // NOTE: シームレスな焦点位置
                    let position = Vec2::ZERO - entity_spec.view_size.center();

                    let entity = entity::Entity::new(entity_spec.id, position);
                    let entity_id = entity_storage.insert(cx, entity).unwrap();

                    *self = Self::Present(Player::new(entity_id, Target::None));
                }
            }
            PlayerSystem::Present(player) => {
                let entity = entity_storage.remove(cx, player.entity_id).unwrap();

                // NOTE: スプリント or 通常
                let speed = if cx.input.key_held(winit::keyboard::KeyCode::ShiftLeft) {
                    Self::SPRINT_SPEED
                } else {
                    Self::DEFAULT_SPEED
                };

                // NOTE: プレイヤーの移動
                let mut move_entity = entity.clone();
                if cx.input.key_held(winit::keyboard::KeyCode::KeyW) {
                    move_entity.position.y += speed * cx.tick.as_secs_f32();
                }
                if cx.input.key_held(winit::keyboard::KeyCode::KeyS) {
                    move_entity.position.y -= speed * cx.tick.as_secs_f32();
                }
                if cx.input.key_held(winit::keyboard::KeyCode::KeyA) {
                    move_entity.position.x -= speed * cx.tick.as_secs_f32();
                }
                if cx.input.key_held(winit::keyboard::KeyCode::KeyD) {
                    move_entity.position.x += speed * cx.tick.as_secs_f32();
                }
                player.entity_id = entity_storage.insert(cx, move_entity).unwrap();

                // NOTE: オブジェクトの選択
                if let (x, y) = cx.input.mouse_diff() {
                    let matrix = camera_sys
                        .get()
                        .world_to_viewport(*cx.window_size)
                        .inverse();

                    let position = matrix.project_point3(vec3(x, y, 0.0)).xy();
                    let bounds = aabb2(position, position);

                    let bases = base_storage
                        .get_by_bounds(cx, base::Bounds::View(bounds))
                        .map(|(id, _)| Target::Base(id));
                    let blocks = block_storage
                        .get_by_bounds(cx, block::Bounds::View(bounds))
                        .map(|(id, _)| Target::Block(id));
                    let entities = entity_storage
                        .get_by_bounds(cx, entity::Bounds::View(bounds))
                        .map(|(id, _)| Target::Entity(id));

                    let mut iter = entities.chain(blocks).chain(bases);
                    if let Some(one) = iter.next() {
                        player.target = one;
                    }
                }
            }
        }
    }
}
