//! システム・ゲームサイクルに関するモジュール

pub use block::BlockSystem;
pub use camera::CameraSystem;
pub use entity::EntitySystem;
pub use generation::GenerationSystem;
pub use player::PlayerSystem;

use glam::*;

mod block;
mod camera;
mod entity;
mod generation;
mod player;

/// 描写処理からのフィードバックデータ
pub struct ReadBack {
    /// 画面座標空間からワールド座標空間への変換行列
    pub screen_to_world_matrix: Option<Mat4>,
    /// 画面座標空間からUI座標空間への変換行列
    pub screen_to_ui_matrix: Option<Mat4>,
}

/// 本ゲームの主要なシステム・ゲームサイクル
pub struct System {
    pub camera: CameraSystem,
    pub block: BlockSystem,
    pub entity: EntitySystem,
    pub generation: GenerationSystem,
    pub player: PlayerSystem,
    time_instant: std::time::Instant,
}

impl System {
    /// 新しいシステムを作成する。
    pub fn new() -> Self {
        Self {
            camera: CameraSystem::default(),
            block: BlockSystem::default(),
            entity: EntitySystem::default(),
            generation: GenerationSystem::default(),
            player: PlayerSystem::default(),
            time_instant: std::time::Instant::now(),
        }
    }

    /// ゲームサイクルを実行する。
    pub fn update(
        &mut self,
        input: &winit_input_helper::WinitInputHelper,
        read_back: Option<&ReadBack>,
    ) {
        let elapsed = self.time_instant.elapsed();
        self.time_instant = std::time::Instant::now();

        if self.player.get_player(&self.entity).is_none() {
            self.player.spawn_player(&mut self.entity);
        }

        if self.camera.get_camera().is_none() {
            self.camera.spawn_camera();
        }

        self.player.update(&mut self.entity, input, elapsed);
        self.camera.update(&self.entity, &self.player, input);

        if let Some(camera) = self.camera.get_camera() {
            self.generation.generate(
                camera.view_bounds().floor().as_iaabb3(),
                &mut self.block,
                &mut self.entity,
            );
        }
    }
}
