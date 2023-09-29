//! ゲームループに関するモジュール

use std::time;

mod camera;
mod entity;
mod generation;
mod player;

pub struct GameLoop {
    pub camera: camera::CameraSystem,
    pub entity: entity::EntitySystem,
    pub generation: generation::GenerationSystem,
    pub player: player::PlayerSystem,
    time_instant: time::Instant,
}

impl GameLoop {
    /// 新しいゲームループを作成する。
    pub fn new() -> Self {
        Self {
            camera: camera::CameraSystem::default(),
            entity: entity::EntitySystem::default(),
            generation: generation::GenerationSystem::default(),
            player: player::PlayerSystem::default(),
            time_instant: time::Instant::now(),
        }
    }

    /// ゲームループを実行する。
    pub fn update(&mut self, input: &winit_input_helper::WinitInputHelper) {
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
            let (start, end) = camera.view_bounds();
            self.generation.generate(start, end, &mut self.entity);
        }
    }
}
