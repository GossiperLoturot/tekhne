//! カメラシステムに関するモジュール

use super::{EntitySystem, PlayerSystem};
use crate::model::*;
use glam::*;

/// ワールド上のカメラの操作を行うシステム
///
/// カメラはワールド内に最大1個のみである。
#[derive(Default)]
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

    /// ワールド上に新しいカメラを作成する。
    ///
    /// ワールド上にカメラが存在せず、作成に成功した場合は`Some(&Camera)`を返す。
    /// 逆にカメラが存在し、作成に失敗した場合は`None`を返す。
    pub fn spawn_camera(&mut self) -> Option<&Camera> {
        if self.camera.is_none() {
            self.camera = Some(Camera::new(Vec3A::ZERO, Self::ZOOM_INIT));
            self.camera.as_ref()
        } else {
            None
        }
    }

    /// ワールド上のカメラを削除する。
    ///
    /// ワールド上にカメラが存在し、削除に成功した場合は`Some(Camera)`を返す。
    /// 逆にカメラが存在せず、削除に失敗した場合は`None`を返す。
    pub fn despawn_camera(&mut self) -> Option<Camera> {
        self.camera.take()
    }

    /// ワールド上のカメラを取得する。
    ///
    /// ワールド上にカメラが存在する場合は`Some(&Camera)`を返す。
    /// 逆にカメラが存在しない場合は`None`を返す。
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
        entity_system: &EntitySystem,
        player_system: &PlayerSystem,
        input: &winit_input_helper::WinitInputHelper,
    ) {
        if let Some(camera) = &mut self.camera {
            if let Some(player) = player_system.get_player(entity_system) {
                camera.position = player.position;
            }

            if input.mouse_pressed(2) {
                camera.zoom = Self::ZOOM_INIT;
            }

            camera.zoom = (camera.zoom + input.scroll_diff()).clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
        }
    }
}
