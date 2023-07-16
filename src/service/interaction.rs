use super::{IUnitService, ReadBack, UnitService};
use crate::model::*;
use glam::*;

#[derive(Debug)]
pub struct IUnitRayHit<'a> {
    point: IVec3,
    iunit: &'a IUnit,
}

#[derive(Debug)]
pub struct UnitRayHit<'a> {
    aabb: Aabb3A,
    units: Vec<&'a Unit>,
}

#[derive(Debug, Default)]
pub struct InteractionService;

impl InteractionService {
    pub fn update(
        &self,
        input: &winit_input_helper::WinitInputHelper,
        read_back: &ReadBack,
        iunit_service: &mut IUnitService,
        unit_service: &mut UnitService,
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
                    if input.mouse_pressed(0) && ray_hit.iunit.breakable() {
                        iunit_service.remove_iunit(ray_hit.point);
                    }
                }

                let ray_hit = Self::unit_ray(start, end, &unit_service);
                if let Some(ray_hit) = ray_hit {
                    if input.mouse_pressed(0) {
                        let id = ray_hit.units[0].id;
                        unit_service.remove_unit(&id);
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
            if let Some(iunit) = iunit_service.get_iunit(point) {
                return Some(IUnitRayHit { point, iunit });
            }
        }

        None
    }

    fn unit_ray(start: Vec3A, end: Vec3A, unit_service: &UnitService) -> Option<UnitRayHit> {
        const FORWARD_STEP: f32 = 0.01;

        let length = (end - start).length() / FORWARD_STEP;
        let delta = (end - start).normalize() * FORWARD_STEP;

        for i in 1..length as usize {
            let aabb = Aabb3A::new(start + delta * i as f32, start + delta * (i - 1) as f32);
            let units = unit_service.get_units(aabb);

            if !units.is_empty() {
                return Some(UnitRayHit { aabb, units });
            }
        }

        None
    }
}
