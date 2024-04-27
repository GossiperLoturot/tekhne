use glam::*;

use crate::aabb::*;
use crate::{assets, game_loop::entity};

pub struct CameraState {
    pub position: Vec2,
    pub zoom: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl CameraState {
    pub fn clipping(&self) -> Aabb2 {
        aabb2(self.position - self.zoom, self.position + self.zoom)
    }

    pub fn world_to_ndc(&self, viewport: (u32, u32)) -> Mat4 {
        let rect = self.clipping();

        // アスペクト比による引き延ばしを補正する行列
        // 描写空間の中に画面が収まるように補正する。
        let (width, height) = viewport;
        let correction = Mat4::from_scale(vec3(
            (height as f32 / width as f32).max(1.0),
            (width as f32 / height as f32).max(1.0),
            1.0,
        ));

        let clipping = Mat4::orthographic_rh(
            rect.min.x,
            rect.max.x,
            rect.min.y,
            rect.max.y,
            self.z_near,
            self.z_far,
        );

        correction * clipping
    }

    pub fn world_to_viewport(&self, viewport: (u32, u32)) -> Mat4 {
        // NDC空間からビューポート空間へ変換する行列
        let (width, height) = viewport;
        let ndc_to_viewport = Mat4::from_scale(vec3(width as f32 * 0.5, height as f32 * 0.5, 1.0))
            * Mat4::from_translation(vec3(1.0, 1.0, 0.0))
            * Mat4::from_scale(vec3(1.0, -1.0, 1.0));

        ndc_to_viewport * self.world_to_ndc(viewport)
    }
}

pub struct CameraSystem {
    assets: std::rc::Rc<assets::Assets>,
    camera_state: CameraState,
}

impl CameraSystem {
    /// 初期位置
    const ORIZIN: Vec2 = Vec2::ZERO;

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

    /// ズームの速さ
    const ZOOM_SPEED: f32 = 16.0;

    #[inline]
    pub fn new(assets: std::rc::Rc<assets::Assets>) -> Self {
        let camera_state = CameraState {
            position: Self::ORIZIN,
            zoom: Self::ZOOM_INIT,
            z_near: Self::Z_NEAR,
            z_far: Self::Z_FAR,
        };

        Self {
            assets,
            camera_state,
        }
    }

    #[inline]
    pub fn get(&self) -> &CameraState {
        &self.camera_state
    }

    pub fn follow_entity(
        &mut self,
        input: &winit_input_helper::WinitInputHelper,
        tick: &std::time::Duration,
        entity_storage: &entity::EntityStorage,
        entity_id: usize,
    ) {
        // NOTE: 視点の追従
        let entity = entity_storage.get(entity_id).unwrap();
        let entity_spec = &self.assets.entity_specs[entity.spec_id];
        self.camera_state.position = entity.position + entity_spec.rendering_size.center();

        // NOTE: 視点の拡大・縮小
        if input.key_held(winit::keyboard::KeyCode::KeyE) {
            self.camera_state.zoom = (self.camera_state.zoom
                + Self::ZOOM_SPEED * tick.as_secs_f32())
            .clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
        }
        if input.key_held(winit::keyboard::KeyCode::KeyQ) {
            self.camera_state.zoom = (self.camera_state.zoom
                - Self::ZOOM_SPEED * tick.as_secs_f32())
            .clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
        }

        // NOTE: 視点の拡大・縮小の初期化
        if input.mouse_held(winit::event::MouseButton::Middle) {
            self.camera_state.zoom = Self::ZOOM_INIT;
        }
    }
}
