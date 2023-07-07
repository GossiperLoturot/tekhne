use super::{IUnitService, ReadBack};
use crate::model::*;
use glam::*;

#[derive(Debug, Default)]
pub struct InteractionService;

impl InteractionService {
    pub fn update(
        &self,
        input: &winit_input_helper::WinitInputHelper,
        read_back: &ReadBack,
        iunit_service: &mut IUnitService,
    ) {
        if let Some(screen_to_world) = read_back.screen_to_world {
            if let Some((x, y)) = input.mouse() {
                if input.mouse_pressed(0) {
                    let start = (screen_to_world * Vec4::new(x as f32, y as f32, 0.0, 1.0)).xyz();
                    let end = (screen_to_world * Vec4::new(x as f32, y as f32, 1.0, 1.0)).xyz();

                    for i in 0..(end - start).length() as i32 {
                        let current = start + (end - start).normalize() * i as f32;
                        let result = iunit_service.remove_iunit(current.floor().as_ivec3());
                        if result.is_some() {
                            break;
                        }
                    }
                }

                if input.mouse_pressed(1) {
                    let start = (screen_to_world * Vec4::new(x as f32, y as f32, 0.0, 1.0)).xyz();
                    let end = (screen_to_world * Vec4::new(x as f32, y as f32, 1.0, 1.0)).xyz();

                    let current = (start + end) * 0.5;
                    iunit_service.add_iunit(IUnit::new(
                        current.floor().as_ivec3(),
                        ResourceKind::SurfaceStone,
                    ));
                }
            }
        }
    }
}
