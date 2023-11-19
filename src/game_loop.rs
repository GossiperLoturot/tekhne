//! ゲームループに関するモジュール

use crate::{assets, renderer};

pub mod base;
pub mod block;
pub mod entity;
pub mod generation;
pub mod player;

pub struct GameLoop {
    pub base: base::BaseSystem,
    pub block: block::BlockSystem,
    pub entity: entity::EntitySystem,
    pub generation: generation::GenerationSystem,
    pub player: player::PlayerSystem,
}

impl GameLoop {
    /// 新しいゲームループを作成する。
    #[inline]
    pub fn new() -> Self {
        Self {
            base: base::BaseSystem::new(),
            block: block::BlockSystem::new(),
            entity: entity::EntitySystem::new(),
            generation: generation::GenerationSystem::new(),
            player: player::PlayerSystem::new(),
        }
    }

    /// ゲームループを実行する。
    pub fn update(
        &mut self,
        assets: &assets::Assets,
        input: &winit_input_helper::WinitInputHelper,
        read_back: &Option<renderer::ReadBack>,
        elapsed: std::time::Duration,
    ) {
        self.player.update(
            assets,
            input,
            read_back,
            elapsed,
            &mut self.base,
            &mut self.block,
            &mut self.entity,
            &mut self.generation,
        );
    }
}
