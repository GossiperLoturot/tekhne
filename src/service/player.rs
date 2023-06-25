use crate::model::{Bounds, Player};
use glam::*;

#[derive(Debug, Default)]
pub struct PlayerService {
    player: Option<Player>,
}

impl PlayerService {
    pub fn spawn_player(&mut self) {
        if self.player.is_some() {
            panic!("player already exists");
        }

        const VIEW_SIZE: f32 = 32.0;
        self.player = Some(Player::new(
            Vec3A::default(),
            Bounds::new(Vec3A::splat(-VIEW_SIZE), Vec3A::splat(VIEW_SIZE)),
            VIEW_SIZE as f32,
        ));
    }

    pub fn get_player(&self) -> Option<&Player> {
        self.player.as_ref()
    }
}
