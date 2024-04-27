//! ゲームループに関するモジュール

use glam::*;

use crate::assets;

mod base;
mod block;
mod camera;
mod entity;
mod generation;
mod player;

pub struct Extract {
    pub matrix: Mat4,
    pub bases: Vec<base::Base>,
    pub blocks: Vec<block::Block>,
    pub entities: Vec<entity::Entity>,
}

pub struct GameLoop {
    base_storage: base::BaseStorage,
    block_storage: block::BlockStorage,
    entity_storage: entity::EntityStorage,
    generation_sys: generation::GenerationSystem,
    camera_sys: camera::CameraSystem,
    player_sys: player::PlayerSystem,
}

impl GameLoop {
    /// 新しいゲームループを作成する。
    #[inline]
    pub fn new(assets: std::rc::Rc<assets::Assets>) -> Self {
        Self {
            base_storage: base::BaseStorage::new(assets.clone()),
            block_storage: block::BlockStorage::new(assets.clone()),
            entity_storage: entity::EntityStorage::new(assets.clone()),
            generation_sys: generation::GenerationSystem::new(assets.clone()),
            camera_sys: camera::CameraSystem::new(assets.clone()),
            player_sys: player::PlayerSystem::new(assets.clone()),
        }
    }

    /// ゲームループを実行する。
    pub fn update(
        &mut self,
        input: &winit_input_helper::WinitInputHelper,
        tick: &std::time::Duration,
        window_size: (u32, u32),
    ) {
        self.player_sys.update(
            input,
            tick,
            window_size,
            &mut self.base_storage,
            &mut self.block_storage,
            &mut self.entity_storage,
            &self.camera_sys,
        );

        // NOTE: カメラの操作
        if let Some(player) = self.player_sys.get_player() {
            self.camera_sys
                .follow_entity(input, tick, &self.entity_storage, player.entity_id);
        }

        // NOTE: 地形の自動生成
        let rect = self.camera_sys.get().clipping();
        self.generation_sys.generate(
            &mut self.base_storage,
            &mut self.block_storage,
            &mut self.entity_storage,
            rect,
        );
    }

    pub fn extract(&self, window_size: (u32, u32)) -> Extract {
        let matrix = self.camera_sys.get().world_to_ndc(window_size);

        let rect = self.camera_sys.get().clipping();

        let bases = self
            .base_storage
            .get_rendering_by_rect(rect)
            .map(|(_, item)| item)
            .cloned()
            .collect::<Vec<_>>();

        let blocks = self
            .block_storage
            .get_rendering_by_rect(rect)
            .map(|(_, item)| item)
            .cloned()
            .collect::<Vec<_>>();

        let entities = self
            .entity_storage
            .get_rendering_by_rect(rect)
            .map(|(_, item)| item)
            .cloned()
            .collect::<Vec<_>>();

        Extract {
            matrix,
            bases,
            blocks,
            entities,
        }
    }
}
