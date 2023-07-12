use crate::model::*;
use glam::*;

#[derive(Debug, Default)]
pub struct PlayerService {
    player: Option<Player>,
}

impl PlayerService {
    pub fn spawn_player(&mut self) {
        if self.player.is_some() {
            panic!("camera already exists");
        }

        self.player = Some(Player::new(Vec3A::Z));
    }

    pub fn update(
        &mut self,
        input: &winit_input_helper::WinitInputHelper,
        elased: std::time::Duration,
    ) {
        const SPEED: f32 = 1.0;
        if let Some(player) = &mut self.player {
            if input.key_held(winit::event::VirtualKeyCode::W) {
                player.position.y += SPEED * elased.as_secs_f32();
            }
            if input.key_held(winit::event::VirtualKeyCode::S) {
                player.position.y -= SPEED * elased.as_secs_f32();
            }
            if input.key_held(winit::event::VirtualKeyCode::A) {
                player.position.x -= SPEED * elased.as_secs_f32();
            }
            if input.key_held(winit::event::VirtualKeyCode::D) {
                player.position.x += SPEED * elased.as_secs_f32();
            }
        }
    }

    pub fn get_player(&self) -> Option<&Player> {
        self.player.as_ref()
    }
}
