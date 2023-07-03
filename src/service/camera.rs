use crate::model::Camera;
use glam::*;

#[derive(Debug, Default)]
enum Control {
    #[default]
    None,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Default)]
pub struct CameraService {
    camera: Option<Camera>,
    control: Control,
}

impl CameraService {
    pub fn spawn_camera(&mut self) {
        if self.camera.is_some() {
            panic!("camera already exists");
        }

        self.camera = Some(Camera::new(Vec3A::ZERO, 16.0));
    }

    pub fn control_camera(&mut self, keyboard: winit::event::KeyboardInput) {
        self.control = Control::None;

        if keyboard.state == winit::event::ElementState::Pressed {
            match keyboard.virtual_keycode {
                Some(winit::event::VirtualKeyCode::W) => {
                    self.control = Control::UP;
                }
                Some(winit::event::VirtualKeyCode::S) => {
                    self.control = Control::DOWN;
                }
                Some(winit::event::VirtualKeyCode::A) => {
                    self.control = Control::LEFT;
                }
                Some(winit::event::VirtualKeyCode::D) => {
                    self.control = Control::RIGHT;
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self, elased: std::time::Duration) {
        if let Some(camera) = &mut self.camera {
            match self.control {
                Control::UP => {
                    camera.position.y += elased.as_secs_f32();
                }
                Control::DOWN => {
                    camera.position.y -= elased.as_secs_f32();
                }
                Control::LEFT => {
                    camera.position.x -= elased.as_secs_f32();
                }
                Control::RIGHT => {
                    camera.position.x += elased.as_secs_f32();
                }
                _ => {}
            }
        }
    }

    pub fn get_camera(&self) -> Option<&Camera> {
        self.camera.as_ref()
    }
}
