use super::{IUnitService, ReadBack};
use crate::model::*;
use glam::*;

#[derive(Debug)]
pub struct IUnitRayHit {
    point: IVec3,
    iunit: IUnit,
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
                let ray_hit = Self::iunit_ray(start, end, iunit_service);

                if let Some(ray_hit) = ray_hit {
                    if input.mouse_pressed(0) && !ray_hit.iunit.resource_kind.unbreakable() {
                        let position = ray_hit.point;
                        iunit_service.remove_iunit(position);
                    }

                    if input.mouse_pressed(1) {
                        let position = ray_hit.point + IVec3::Z;
                        iunit_service.add_iunit(IUnit::new(position, ResourceKind::SurfaceStone));
                    }
                }
            }
        }
    }

    fn iunit_ray(start: Vec3A, end: Vec3A, iunit_service: &IUnitService) -> Option<IUnitRayHit> {
        const FORWARD_STEP: f32 = 0.01;

        let length = (end - start).length() / FORWARD_STEP;
        let delta = (end - start).normalize() * FORWARD_STEP;

        for i in 0..length as usize {
            let point = (start + delta * i as f32).round().as_ivec3();
            if let Some(iunit) = iunit_service.get_iunit(point).cloned() {
                return Some(IUnitRayHit { point, iunit });
            }
        }

        None
    }
}
