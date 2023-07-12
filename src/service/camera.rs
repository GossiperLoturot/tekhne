use super::PlayerService;
use crate::model::*;
use glam::*;

#[derive(Debug, Default)]
pub struct CameraService {
    camera: Option<Camera>,
}

impl CameraService {
    const ZOOM_INIT: f32 = 16.0;
    const ZOOM_MIN: f32 = 4.0;
    const ZOOM_MAX: f32 = 128.0;

    pub fn spawn_camera(&mut self) {
        if self.camera.is_some() {
            panic!("camera already exists");
        }

        self.camera = Some(Camera::new(Vec3A::ZERO, Self::ZOOM_INIT));
    }

    pub fn update(
        &mut self,
        player_service: &PlayerService,
        input: &winit_input_helper::WinitInputHelper,
    ) {
        if let Some(camera) = &mut self.camera {
            if let Some(player) = player_service.get_player() {
                camera.position = player.position;
            }

            if input.mouse_pressed(2) {
                camera.zoom = Self::ZOOM_INIT;
            }

            camera.zoom = (camera.zoom + input.scroll_diff()).clamp(Self::ZOOM_MIN, Self::ZOOM_MAX);
        }
    }

    pub fn get_camera(&self) -> Option<&Camera> {
        self.camera.as_ref()
    }
}
