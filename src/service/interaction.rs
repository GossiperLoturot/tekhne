use super::{IUnitService, ReadBack};
use crate::model::*;
use glam::*;

#[derive(Debug)]
pub struct IUnitRayHit {
    inside_point: IVec3,
    outside_point: IVec3,
}

#[derive(Debug, Default)]
pub struct InteractionService;

impl InteractionService {
    pub fn update(
        &self,
        input: &winit_input_helper::WinitInputHelper,
        read_back: &ReadBack,
        iunit_service: &mut IUnitService,
    ) {
        if let Some(matrix) = read_back.screen_to_world_matrix {
            if let Some((x, y)) = input.mouse() {
                let start = (matrix * Vec4::new(x as f32, y as f32, 0.0, 1.0))
                    .xyz()
                    .into();
                let end = (matrix * Vec4::new(x as f32, y as f32, 1.0, 1.0))
                    .xyz()
                    .into();
                let hit_info = Self::iunit_ray(start, end, iunit_service);

                if let Some(hit_info) = hit_info {
                    if input.mouse_pressed(0) {
                        let position = hit_info.inside_point;
                        iunit_service.remove_iunit(position);
                    }

                    if input.mouse_pressed(1) {
                        let position = hit_info.outside_point;
                        iunit_service.add_iunit(IUnit::new(position, ResourceKind::SurfaceStone));
                    }
                }
            }
        }
    }

    fn iunit_ray(start: Vec3A, end: Vec3A, iunit_service: &IUnitService) -> Option<IUnitRayHit> {
        let length = (end - start).length();
        let delta = (end - start).normalize();

        for i in 0..length as usize {
            let point = (start + delta * i as f32).round().as_ivec3();
            if iunit_service.get_iunit(point).is_some() {
                return Some(IUnitRayHit {
                    inside_point: point,
                    outside_point: (start + delta * (i - 1) as f32).round().as_ivec3(),
                });
            }
        }

        None
    }
}
