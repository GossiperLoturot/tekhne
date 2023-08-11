use super::UnitService;
use crate::model::*;
use glam::*;

#[derive(Default)]
pub struct PlayerService {
    player: Option<usize>,
}

impl PlayerService {
    const DEFAULT_SPEED: f32 = 2.0;
    const SPRINT_SPEED: f32 = 4.0;

    pub fn spawn_player(&mut self, unit_service: &mut UnitService) {
        if let Some(id) = self.player {
            panic!("player {} already exists", id);
        } else {
            let id = unit_service.add_unit(Unit::new(vec3a(0.0, 0.0, 1.0), UnitKind::Player));
            self.player = Some(id);
        }
    }

    pub fn update(
        &mut self,
        unit_service: &mut UnitService,
        input: &winit_input_helper::WinitInputHelper,
        elased: std::time::Duration,
    ) {
        if let Some(id) = self.player {
            if let Some(mut player) = unit_service.remove_unit(id) {
                let speed = if input.key_held(winit::event::VirtualKeyCode::LShift) {
                    Self::SPRINT_SPEED
                } else {
                    Self::DEFAULT_SPEED
                };

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

                unit_service.add_unit(player);
            }
        }
    }

    pub fn get_player<'a>(&self, unit_service: &'a UnitService) -> Option<&'a Unit> {
        if let Some(id) = self.player {
            unit_service.get_unit(id)
        } else {
            None
        }
    }
}
