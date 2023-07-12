use super::PlayerService;
use crate::model::*;
use glam::*;

#[derive(Debug, Default)]
pub struct CameraService {
    camera: Option<Camera>,
}

impl CameraService {
    pub fn spawn_camera(&mut self) {
        if self.camera.is_some() {
            panic!("camera already exists");
        }

        self.camera = Some(Camera::new(Vec3A::ZERO, 16.0));
    }

    pub fn update(&mut self, player_service: &PlayerService) {
        if let Some(camera) = &mut self.camera {
            if let Some(player) = player_service.get_player() {
                camera.position = player.position;
            }
        }
    }

    pub fn get_camera(&self) -> Option<&Camera> {
        self.camera.as_ref()
    }
}
