//! ゲームループに関するモジュール

use std::time;

use aabb::*;

use crate::assets;

pub mod block;
pub mod camera;
pub mod entity;
pub mod generation;
pub mod player;

pub struct GameLoop {
    pub camera: camera::CameraSystem,
    pub entity: entity::EntitySystem,
    pub block: block::BlockSystem,
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
            entity: entity::EntitySystem::new(),
            block: block::BlockSystem::new(),
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

        self.player.update(assets, &mut self.entity, input, elapsed);
        self.camera.update(&self.entity, &self.player, input);

        if let Some(camera) = self.camera.get_camera() {
            let bounds = camera.view_bounds();
            // TODO: generates entity

            let bounds = aabb2(bounds.min.floor(), bounds.max.ceil()).as_iaabb2();
            self.generation.generate(assets, &mut self.block, bounds);
        }
    }
}
