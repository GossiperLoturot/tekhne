use crate::{models::*, services::IUnitService};
use glam::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct GenerationClient {
    id: String,
    bounds: IBounds3,
}

impl GenerationClient {
    pub fn new(id: String, bounds: IBounds3) -> Self {
        Self { id, bounds }
    }
}

#[derive(Debug, Default)]
pub struct GenerationService {
    init_flags: HashSet<IVec2>,
    clients: HashMap<String, GenerationClient>,
}

impl GenerationService {
    pub fn set_bounds(
        &mut self,
        client_name: &str,
        bounds: IBounds3,
        iunit_service: &mut IUnitService,
    ) {
        if match self.clients.get(client_name) {
            Some(client) => client.bounds != bounds,
            None => true,
        } {
            for x in bounds.min.x..=bounds.max.x {
                for y in bounds.min.y..=bounds.max.y {
                    let pos = IVec2::new(x, y);
                    if !self.init_flags.contains(&pos) {
                        // generation rules start

                        let iunit = IUnit::new(IVec3::new(pos.x, pos.y, 0), "Surface".to_string());
                        iunit_service.add_iunit(iunit);

                        // generation rules end

                        self.init_flags.insert(pos);
                    }
                }
            }

            let client = GenerationClient::new(client_name.to_string(), bounds);
            self.clients.insert(client_name.to_string(), client);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_bounds() {
        let mut iunit_service = IUnitService::default();
        let mut gen_service = GenerationService::default();

        let bounds = IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        gen_service.set_bounds("TEST_CLIENT_NAME", bounds, &mut iunit_service);
    }
}
