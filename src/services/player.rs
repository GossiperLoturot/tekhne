use crate::models::{IBounds3, Player};
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

        // temporary code
        const VIEW_SIZE: i32 = 32;
        self.player = Some(Player::new(
            Vec3A::default(),
            IBounds3::new(IVec3::splat(-VIEW_SIZE), IVec3::splat(VIEW_SIZE)),
            VIEW_SIZE as f32,
        ));
    }

    pub fn get_player(&self) -> Option<&Player> {
        self.player.as_ref()
    }
}
