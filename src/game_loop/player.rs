//! プレイヤーシステムの機能に関するモジュール

use aabb::*;
use glam::*;

use crate::{
    assets,
    game_loop::{base, block, entity, generation},
    renderer,
};

/// 選択しているオブジェクト
pub enum Cursor {
    None,
    Base(usize),
    Block(usize),
    Entity(usize),
}

/// プレイヤーシステムの機能
pub enum PlayerSystem {
    NotPresent,
    Spawn {
        player_spec_id: usize,
    },
    Present {
        entity_id: usize,
        focus: Vec2,
        zoom: f32,
        cursor: Cursor,
    },
}

impl PlayerSystem {
    /// 通常時の移動速度
    const DEFAULT_SPEED: f32 = 2.0;

    /// スプリント時の移動速度
    const SPRINT_SPEED: f32 = 4.0;

    /// 拡大・縮小の初期値
    const ZOOM_INIT: f32 = 16.0;

    /// 拡大・縮小の最小値
    const ZOOM_MIN: f32 = 4.0;

    /// 拡大・縮小の最大値
    const ZOOM_MAX: f32 = 128.0;

    /// Z値クリップの最小値
    const Z_NEAR: f32 = -32.0;

    /// Z値クリップの最大値
    const Z_FAR: f32 = 32.0;

    /// 新しいプレイヤーシステムを作成する。
    #[inline]
    pub fn new() -> Self {
        Self::NotPresent
    }

    /// ゲームサイクルにおける振る舞いを実行する。
    pub fn update(
        &mut self,
        assets: &assets::Assets,
        input: &winit_input_helper::WinitInputHelper,
        read_back: &Option<renderer::ReadBack>,
        elased: std::time::Duration,
        base_system: &mut base::BaseSystem,
        block_system: &mut block::BlockSystem,
        entity_system: &mut entity::EntitySystem,
        generation_system: &mut generation::GenerationSystem,
    ) {
        match self {
            PlayerSystem::NotPresent => {
                if input.key_pressed(winit::keyboard::KeyCode::Enter) {
                    *self = Self::Spawn { player_spec_id: 0 };
                }
            }
            PlayerSystem::Spawn { player_spec_id } => {
                if input.key_pressed(winit::keyboard::KeyCode::KeyA) {
                    *player_spec_id = player_spec_id
                        .saturating_sub(1)
                        .clamp(0, assets.player_specs.len() - 1);
                }

                if input.key_pressed(winit::keyboard::KeyCode::KeyD) {
                    *player_spec_id = player_spec_id
                        .saturating_add(1)
                        .clamp(0, assets.player_specs.len() - 1);
                }

                // NOTE: プレイヤーをワールド上に作成する。
                if input.key_pressed(winit::keyboard::KeyCode::Enter) {
                    let player_spec = &assets.player_specs[*player_spec_id];
                    let entity_spec = &assets.entity_specs[player_spec.entity_spec_id];

                    let spec_id = player_spec.entity_spec_id;
                    let position = Vec2::ZERO;
                    let entity = entity::Entity::new(spec_id, position);

                    let cursor = Cursor::None;
                    let zoom = Self::ZOOM_INIT;
                    let focus = position + entity_spec.view_size.center();
                    let entity_id = entity_system.insert(assets, entity).unwrap();
                    *self = Self::Present {
                        entity_id,
                        focus,
                        zoom,
                        cursor,
                    };
                }
            }
            PlayerSystem::Present {
                entity_id,
                focus,
                zoom,
                cursor,
            } => {
                // NOTE: スプリント or 通常
                let speed = if input.key_held(winit::keyboard::KeyCode::ShiftLeft) {
                    Self::SPRINT_SPEED
                } else {
                    Self::DEFAULT_SPEED
                };

                let mut entity = entity_system.remove(assets, *entity_id).unwrap();

                // NOTE: WSAD移動操作
                if input.key_held(winit::keyboard::KeyCode::KeyW) {
                    entity.position.y += speed * elased.as_secs_f32();
                }
                if input.key_held(winit::keyboard::KeyCode::KeyS) {
                    entity.position.y -= speed * elased.as_secs_f32();
                }
                if input.key_held(winit::keyboard::KeyCode::KeyA) {
                    entity.position.x -= speed * elased.as_secs_f32();
                }
                if input.key_held(winit::keyboard::KeyCode::KeyD) {
                    entity.position.x += speed * elased.as_secs_f32();
                }

                // PANIC: エンティティ設置に失敗する可能性あり。
                *entity_id = entity_system.insert(assets, entity.clone()).unwrap();

                // NOTE: 視点の追従
                let entity_spec = &assets.entity_specs[entity.spec_id];
                *focus = entity.position + entity_spec.view_size.center();

                // NOTE: 視点のズーム
                let (_, y_scroll) = input.scroll_diff();
                *zoom = (*zoom + y_scroll).clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);

                // NOTE: 地形の自動生成
                let bounds = aabb2(*focus - *zoom, *focus + *zoom);
                generation_system.update(assets, base_system, block_system, entity_system, bounds);

                // NOTE: オブジェクトの選択
                if let Some(renderer::ReadBack {
                    screen_to_world_matrix: Some(matrix),
                }) = read_back
                {
                    if let Some((x, y)) = input.cursor() {
                        let position = matrix.project_point3(vec3(x, y, 0.0)).xy();
                        let bounds = aabb2(position, position);

                        let entities = entity_system
                            .get_by_bounds(assets, entity::Bounds::View(bounds))
                            .map(|(id, _)| Cursor::Entity(id));
                        let blocks = block_system
                            .get_by_bounds(assets, block::Bounds::View(bounds))
                            .map(|(id, _)| Cursor::Block(id));
                        let bases = base_system
                            .get_by_bounds(base::Bounds::View(bounds))
                            .map(|(id, _)| Cursor::Base(id));

                        let mut iter = entities.chain(blocks).chain(bases);
                        if let Some(one) = iter.next() {
                            *cursor = one;
                        }
                    }
                }

                // NOTE: 選択したオブジェクトの削除操作
                if input.mouse_pressed(0) {
                    match cursor {
                        Cursor::Block(id) => {
                            block_system.remove(assets, *id);
                        }
                        Cursor::Entity(id) => {
                            entity_system.remove(assets, *id);
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    /// 描写範囲を返す。
    #[inline]
    pub fn view_bounds(&self) -> Option<Aabb2> {
        match self {
            PlayerSystem::Present { focus, zoom, .. } => {
                Some(aabb2(*focus - *zoom, *focus + *zoom))
            }
            _ => None,
        }
    }

    /// 描写範囲のビュー行列を返す。
    #[inline]
    pub fn view_matrix(&self) -> Option<Mat4> {
        match self.view_bounds() {
            Some(bounds) => Some(Mat4::orthographic_rh(
                bounds.min.x,
                bounds.max.x,
                bounds.min.y,
                bounds.max.y,
                Self::Z_NEAR,
                Self::Z_FAR,
            )),
            None => None,
        }
    }
}
