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

    pub fn update(
        &mut self,
        input: &winit_input_helper::WinitInputHelper,
        elased: std::time::Duration,
    ) {
        const SPEED: f32 = 1.0;
        if let Some(camera) = &mut self.camera {
            if input.key_held(winit::event::VirtualKeyCode::W) {
                camera.position.y += SPEED * elased.as_secs_f32();
            }
            if input.key_held(winit::event::VirtualKeyCode::S) {
                camera.position.y -= SPEED * elased.as_secs_f32();
            }
            if input.key_held(winit::event::VirtualKeyCode::A) {
                camera.position.x -= SPEED * elased.as_secs_f32();
            }
            if input.key_held(winit::event::VirtualKeyCode::D) {
                camera.position.x += SPEED * elased.as_secs_f32();
            }
        }
    }

    pub fn get_camera(&self) -> Option<&Camera> {
        self.camera.as_ref()
    }
}
