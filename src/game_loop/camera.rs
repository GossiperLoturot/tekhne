use aabb::*;
use glam::*;

use crate::game_loop::{self, base, block, entity};

/// 追従しているオブジェクト
pub enum Target {
    Base(usize),
    Block(usize),
    Entity(usize),
}

pub struct Camera {
    position: Vec2,
    zoom: f32,
}

impl Camera {
    /// Z値クリップの最小値
    const Z_NEAR: f32 = -32.0;

    /// Z値クリップの最大値
    const Z_FAR: f32 = 32.0;

    /// 新しいカメラシステムを作成する。
    #[inline]
    pub fn new(position: Vec2, zoom: f32) -> Self {
        Self { position, zoom }
    }

    /// 描写範囲を返す。
    #[inline]
    pub fn view_bounds(&self) -> Aabb2 {
        aabb2(self.position - self.zoom, self.position + self.zoom)
    }

    /// 描写範囲のビュー行列を返す。
    #[inline]
    pub fn view_matrix(&self) -> Mat4 {
        let bounds = self.view_bounds();
        Mat4::orthographic_rh(
            bounds.min.x,
            bounds.max.x,
            bounds.min.y,
            bounds.max.y,
            Self::Z_NEAR,
            Self::Z_FAR,
        )
    }
}

pub struct CameraSystem {
    camera: Camera,
}

impl CameraSystem {
    /// 拡大・縮小の初期値
    const ZOOM_INIT: f32 = 16.0;

    /// 拡大・縮小の最小値
    const ZOOM_MIN: f32 = 4.0;

    /// 拡大・縮小の最大値
    const ZOOM_MAX: f32 = 128.0;

    /// 移動の速さ
    const MOVE_SPEED: f32 = 4.0;

    /// ズームの速さ
    const ZOOM_SPEED: f32 = 30.0;

    #[inline]
    pub fn new() -> Self {
        Self {
            camera: Camera::new(Vec2::ZERO, Self::ZOOM_INIT),
        }
    }

    #[inline]
    pub fn get(&self) -> &Camera {
        &self.camera
    }

    pub fn free_look(&mut self, cx: &game_loop::InputContext) {
        // NOTE: 視点の移動
        if cx.input.key_held(winit::keyboard::KeyCode::KeyW) {
            self.camera.position.y += Self::MOVE_SPEED * cx.elapsed.as_secs_f32();
        }
        if cx.input.key_held(winit::keyboard::KeyCode::KeyS) {
            self.camera.position.y -= Self::MOVE_SPEED * cx.elapsed.as_secs_f32();
        }
        if cx.input.key_held(winit::keyboard::KeyCode::KeyA) {
            self.camera.position.x -= Self::MOVE_SPEED * cx.elapsed.as_secs_f32();
        }
        if cx.input.key_held(winit::keyboard::KeyCode::KeyD) {
            self.camera.position.x += Self::MOVE_SPEED * cx.elapsed.as_secs_f32();
        }

        // NOTE: 視点の拡大・縮小
        let (_, y_scroll) = cx.input.scroll_diff();
        self.camera.zoom = (self.camera.zoom
            + y_scroll * Self::ZOOM_SPEED * cx.elapsed.as_secs_f32())
        .clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);

        // NOTE: 視点の拡大・縮小の初期化
        if cx.input.mouse_held(2) {
            self.camera.zoom = Self::ZOOM_INIT;
        }
    }

    pub fn follow(
        &mut self,
        cx: &game_loop::InputContext,
        base_storage: &mut base::BaseStorage,
        block_storage: &block::BlockStorage,
        entity_storage: &entity::EntityStorage,
        target: Target,
    ) {
        // NOTE: 視点の追従
        match target {
            Target::Base(id) => {
                let obj = base_storage.get(id).unwrap();
                self.camera.position = obj.position.as_vec2() + Vec2::splat(0.5);
            }
            Target::Block(id) => {
                let obj = block_storage.get(id).unwrap();
                let obj_spec = &cx.assets.block_specs[obj.spec_id];
                self.camera.position = obj.position.as_vec2() + obj_spec.view_size.center();
            }
            Target::Entity(id) => {
                let obj = entity_storage.get(id).unwrap();
                let obj_spec = &cx.assets.entity_specs[obj.spec_id];
                self.camera.position = obj.position + obj_spec.view_size.center();
            }
        }

        // NOTE: 視点の拡大・縮小
        let (_, y_scroll) = cx.input.scroll_diff();
        self.camera.zoom = (self.camera.zoom
            + y_scroll * Self::ZOOM_SPEED * cx.elapsed.as_secs_f32())
        .clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);

        // NOTE: 視点の拡大・縮小の初期化
        if cx.input.mouse_held(2) {
            self.camera.zoom = Self::ZOOM_INIT;
        }
    }
}
