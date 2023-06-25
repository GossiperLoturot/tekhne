use crate::{model::*, service::IUnitService};
use glam::*;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct GenerationService {
    init_flags: HashSet<IVec2>,
}

impl GenerationService {
    pub fn generate(&mut self, bounds: Bounds<IVec3>, iunit_service: &mut IUnitService) {
        for x in bounds.min.x..=bounds.max.x {
            for y in bounds.min.y..=bounds.max.y {
                let pos = IVec2::new(x, y);
                if !self.init_flags.contains(&pos) {
                    // generation rules start

                    iunit_service.add_iunit(IUnit::new(IVec3::new(x, y, 0), "Surface".to_string()));

                    // generation rules end

                    self.init_flags.insert(pos);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate() {
        let mut iunit_service = IUnitService::default();
        let mut gen_service = GenerationService::default();

        let bounds = Bounds::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        gen_service.generate(bounds, &mut iunit_service);
    }
}
