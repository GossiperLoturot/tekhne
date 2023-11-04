//! カメラの機能に関するモジュール

use aabb::*;
use glam::*;

use crate::game_loop::{entity, player};

pub struct Camera {
    pub position: Vec2,
    pub zoom: f32,
}

impl Camera {
    const NEAR: f32 = -128.0;
    const FAR: f32 = 128.0;

    /// 新しいカメラを作成する。
    #[inline]
    pub fn new(position: Vec2, zoom: f32) -> Self {
        Self { position, zoom }
    }

    /// 描写領域を含むようなビュー行列を返す。
    #[inline]
    pub fn view_matrix(&self) -> Mat4 {
        let view_aabb = self.view_bounds();
        Mat4::orthographic_rh(
            view_aabb.min.x,
            view_aabb.max.x,
            view_aabb.min.y,
            view_aabb.max.y,
            Self::NEAR,
            Self::FAR,
        )
    }

    /// 描写範囲を返す。
    #[inline]
    pub fn view_bounds(&self) -> Aabb2 {
        Aabb2::new(
            self.position - Vec2::splat(self.zoom),
            self.position + Vec2::splat(self.zoom),
        )
    }
}

/// カメラの機能
pub struct CameraSystem {
    camera: Option<Camera>,
}

impl CameraSystem {
    /// 拡大・縮小の初期値
    const ZOOM_INIT: f32 = 16.0;

    /// 拡大・縮小の最小値
    const ZOOM_MIN: f32 = 4.0;

    /// 拡大・縮小の最大値
    const ZOOM_MAX: f32 = 128.0;

    #[inline]
    pub fn new() -> Self {
        Self {
            camera: Default::default(),
        }
    }

    /// カメラを作成する。
    #[inline]
    pub fn spawn_camera(&mut self) -> Option<&Camera> {
        if self.camera.is_none() {
            self.camera = Some(Camera::new(Vec2::ZERO, Self::ZOOM_INIT));
            self.camera.as_ref()
        } else {
            None
        }
    }

    /// カメラを削除する。
    #[inline]
    pub fn despawn_camera(&mut self) -> Option<Camera> {
        self.camera.take()
    }

    /// カメラを取得する。
    #[inline]
    pub fn get_camera(&self) -> Option<&Camera> {
        self.camera.as_ref()
    }

    /// ゲームサイクルにおけるカメラの振る舞いを実行する。
    ///
    /// 振る舞いは以下のとおりである。
    ///
    /// - カメラはプレイヤーに追従する。
    /// - マウスホイールで拡大・縮小を行う。
    /// - ホイールクリックで拡大・縮小をリセットする。
    pub fn update(
        &mut self,
        entity_system: &entity::EntitySystem,
        player_system: &player::PlayerSystem,
        input: &winit_input_helper::WinitInputHelper,
    ) {
        if let Some(camera) = &mut self.camera {
            if let Some(player) = player_system.get_player(entity_system) {
                camera.position = player.position;
            }

            if input.mouse_pressed(2) {
                camera.zoom = Self::ZOOM_INIT;
            }

            let (_, y_scroll) = input.scroll_diff();
            camera.zoom = (camera.zoom + y_scroll).clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
        }
    }
}
