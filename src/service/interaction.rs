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
                let position = discreted_ray(
                    (screen_to_world * Vec4::new(x as f32, y as f32, 0.0, 1.0)).xyz(),
                    (screen_to_world * Vec4::new(x as f32, y as f32, 1.0, 1.0)).xyz(),
                    |position| iunit_service.get_iunit(position).is_some(),
                );

                if let Some((position, prev_position)) = position {
                    if input.mouse_pressed(0) {
                        iunit_service.remove_iunit(position);
                    }

                    if let Some(prev_position) = prev_position {
                        if input.mouse_pressed(1) {
                            iunit_service
                                .add_iunit(IUnit::new(prev_position, ResourceKind::SurfaceStone));
                        }
                    }
                }
            }
        }
    }
}

const STEP_PER_LENGTH: f32 = 2.0;
fn discreted_ray<F>(start: Vec3, end: Vec3, collision: F) -> Option<(IVec3, Option<IVec3>)>
where
    F: Fn(IVec3) -> bool,
{
    let diff = end - start;
    let dir = diff.normalize();

    let mut prev_position = None;
    let iteration = (STEP_PER_LENGTH * diff.length()) as u32;
    for i in 0..iteration {
        let position = (start + dir * i as f32).floor().as_ivec3();

        if Some(position) == prev_position {
            continue;
        }

        if collision(position) {
            return Some((position, prev_position));
        }

        prev_position = Some(position);
    }

    None
}
