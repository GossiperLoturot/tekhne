//! ゲームループに関するモジュール

use std::time;

use crate::assets;

pub mod base;
pub mod block;
pub mod camera;
pub mod entity;
pub mod generation;
pub mod player;

pub struct GameLoop {
    pub camera: camera::CameraSystem,
    pub base: base::BaseSystem,
    pub block: block::BlockSystem,
    pub entity: entity::EntitySystem,
    pub generation: generation::GenerationSystem,
    pub player: player::PlayerSystem,
    pub time_instant: time::Instant,
}

impl GameLoop {
    /// 新しいゲームループを作成する。
    #[inline]
    pub fn new() -> Self {
        Self {
            camera: camera::CameraSystem::new(),
            base: base::BaseSystem::new(),
            block: block::BlockSystem::new(),
            entity: entity::EntitySystem::new(),
            generation: generation::GenerationSystem::new(),
            player: player::PlayerSystem::new(),
            time_instant: time::Instant::now(),
        }
    }

    /// ゲームループを実行する。
    pub fn update(
        &mut self,
        assets: &assets::Assets,
        input: &winit_input_helper::WinitInputHelper,
    ) {
        let elapsed = self.time_instant.elapsed();
        self.time_instant = std::time::Instant::now();

        if self.player.get_player(&self.entity).is_none() {
            self.player.spawn_player(assets, &mut self.entity);
        }

        if self.camera.get_camera().is_none() {
            self.camera.spawn_camera();
        }

        self.player.update(assets, input, &mut self.entity, elapsed);
        self.camera.update(input, &self.entity, &self.player);

        if let Some(camera) = self.camera.get_camera() {
            self.generation.generate(
                assets,
                &mut self.base,
                &mut self.block,
                &mut self.entity,
                camera.view_bounds(),
            );
        }
    }
}
