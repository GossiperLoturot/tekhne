use super::{IUnitService, ReadBack, UnitService};
use crate::model::*;
use glam::*;

const FORWARD_STEP: f32 = 0.01;

#[derive(Debug, Clone)]
pub struct IUnitRayHit<'a> {
    point: IVec3,
    iunit: &'a IUnit,
}

#[derive(Debug, Clone)]
pub struct UnitRayHit<'a> {
    aabb: Aabb3A,
    units: Vec<&'a Unit>,
}

fn iunit_ray(start: Vec3A, end: Vec3A, iunit_service: &IUnitService) -> Option<IUnitRayHit> {
    let length = (end - start).length() / FORWARD_STEP;
    let delta = (end - start).normalize() * FORWARD_STEP;

    for i in 0..length as usize {
        let point = (start + delta * i as f32).floor().as_ivec3();

        if let Some(iunit) = iunit_service.get_iunit(point) {
            return Some(IUnitRayHit { point, iunit });
        }
    }

    None
}

fn unit_ray(start: Vec3A, end: Vec3A, unit_service: &UnitService) -> Option<UnitRayHit> {
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

#[derive(Default)]
pub struct InteractionService {}

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
                let start = (matrix * Vec4::new(x, y, 0.0, 1.0)).xyz().into();
                let end = (matrix * Vec4::new(x, y, 1.0, 1.0)).xyz().into();

                if let Some(hit) = iunit_ray(start, end, iunit_service) {
                    if input.mouse_pressed(0) && hit.iunit.breakable() {
                        iunit_service.remove_iunit(hit.iunit.position);
                    }
                }

                if let Some(hit) = unit_ray(start, end, unit_service) {
                    if input.mouse_pressed(0) && hit.units[0].breakable() {
                        unit_service.remove_unit(&hit.units[0].id.clone());
                    }
                }
            }
        }
    }
}
