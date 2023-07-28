use super::{PlayerService, UnitService};
use crate::model::*;
use glam::*;

#[derive(Default)]
pub struct CameraService {
    camera: Option<Camera>,
}

impl CameraService {
    const ZOOM_INIT: f32 = 16.0;
    const ZOOM_MIN: f32 = 4.0;
    const ZOOM_MAX: f32 = 128.0;

    pub fn spawn_camera(&mut self) {
        if let Some(camera) = &self.camera {
            panic!("camera {:?} already exists", camera);
        } else {
            self.camera = Some(Camera::new(Vec3A::ZERO, Self::ZOOM_INIT));
        }
    }

    pub fn update(
        &mut self,
        unit_service: &UnitService,
        player_service: &PlayerService,
        input: &winit_input_helper::WinitInputHelper,
    ) {
        if let Some(camera) = &mut self.camera {
            if let Some(player) = player_service.get_player(unit_service) {
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
