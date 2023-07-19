use crate::model::*;
use glam::*;

#[derive(Default)]
pub struct PlayerService {
    player: Option<Player>,
}

impl PlayerService {
    const DEFAULT_SPEED: f32 = 2.0;
    const SPRINT_SPEED: f32 = 4.0;

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
        let speed = if input.key_held(winit::event::VirtualKeyCode::LShift) {
            Self::SPRINT_SPEED
        } else {
            Self::DEFAULT_SPEED
        };

        if let Some(player) = &mut self.player {
            if input.key_held(winit::event::VirtualKeyCode::W) {
                player.position.y += speed * elased.as_secs_f32();
            }
            if input.key_held(winit::event::VirtualKeyCode::S) {
                player.position.y -= speed * elased.as_secs_f32();
            }
            if input.key_held(winit::event::VirtualKeyCode::A) {
                player.position.x -= speed * elased.as_secs_f32();
            }
            if input.key_held(winit::event::VirtualKeyCode::D) {
                player.position.x += speed * elased.as_secs_f32();
            }
        }
    }

    pub fn get_player(&self) -> Option<&Player> {
        self.player.as_ref()
    }
}
