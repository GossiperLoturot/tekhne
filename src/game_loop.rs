//! ゲームループに関するモジュール

use glam::*;

use crate::{assets, game_loop};

mod base;
mod block;
mod camera;
mod entity;
mod generation;
mod player;

pub struct Context<'a> {
    pub assets: &'a assets::Assets,
    pub input: &'a winit_input_helper::WinitInputHelper,
    pub tick: &'a std::time::Duration,
    pub window_size: &'a (u32, u32),
}

pub struct GameExtract {
    pub matrix: Mat4,
    pub bases: Vec<base::Base>,
    pub blocks: Vec<block::Block>,
    pub entities: Vec<entity::Entity>,
}

pub struct GameLoop {
    pub base_storage: base::BaseStorage,
    pub block_storage: block::BlockStorage,
    pub entity_storage: entity::EntityStorage,
    pub generation_sys: generation::GenerationSystem,
    pub camera_sys: camera::CameraSystem,
    pub player_sys: player::PlayerSystem,
}

impl GameLoop {
    /// 新しいゲームループを作成する。
    #[inline]
    pub fn new() -> Self {
        Self {
            base_storage: base::BaseStorage::new(),
            block_storage: block::BlockStorage::new(),
            entity_storage: entity::EntityStorage::new(),
            generation_sys: generation::GenerationSystem::new(),
            camera_sys: camera::CameraSystem::new(),
            player_sys: player::PlayerSystem::new(),
        }
    }

    /// ゲームループを実行する。
    pub fn update(&mut self, cx: &game_loop::Context) {
        self.player_sys.update(
            cx,
            &mut self.base_storage,
            &mut self.block_storage,
            &mut self.entity_storage,
            &self.camera_sys,
        );

        // NOTE: カメラの操作
        if let player::PlayerSystem::Present(player) = &self.player_sys {
            let target = camera::Target::Entity(player.entity_id);
            self.camera_sys.follow(
                cx,
                &self.base_storage,
                &self.block_storage,
                &self.entity_storage,
                target,
            );
        }

        // NOTE: 地形の自動生成
        let bounds = self.camera_sys.get().clipping();
        self.generation_sys.generate(
            cx,
            &mut self.base_storage,
            &mut self.block_storage,
            &mut self.entity_storage,
            bounds,
        );
    }

    pub fn extract(&self, cx: &game_loop::Context) -> GameExtract {
        let matrix = self.camera_sys.get().world_to_ndc(*cx.window_size);

        let bounds = self.camera_sys.get().clipping();

        let bases = self
            .base_storage
            .get_by_bounds(cx, base::Bounds::View(bounds))
            .map(|(_, item)| item)
            .cloned()
            .collect::<Vec<_>>();

        let blocks = self
            .block_storage
            .get_by_bounds(cx, block::Bounds::View(bounds))
            .map(|(_, item)| item)
            .cloned()
            .collect::<Vec<_>>();

        let entities = self
            .entity_storage
            .get_by_bounds(cx, entity::Bounds::View(bounds))
            .map(|(_, item)| item)
            .cloned()
            .collect::<Vec<_>>();

        GameExtract {
            matrix,
            bases,
            blocks,
            entities,
        }
    }
}
