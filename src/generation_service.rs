use std::collections::{HashMap, HashSet};

use glam::{IVec2, IVec3};

use crate::{
    models::{IBounds3, Tile},
    tile_service::TileService,
};

pub struct GenerationClient {
    bounds: IBounds3,
}

impl GenerationClient {
    pub fn new(bounds: IBounds3) -> Self {
        Self { bounds }
    }
}

pub struct GenerationService {
    init_flags: HashSet<IVec2>,
    clients: HashMap<String, GenerationClient>,
}

impl GenerationService {
    pub fn new() -> Self {
        Self {
            init_flags: HashSet::new(),
            clients: HashMap::new(),
        }
    }

    pub fn set_bounds(
        &mut self,
        client_name: String,
        bounds: IBounds3,
        tile_service: &mut TileService,
    ) {
        if match self.clients.get(&client_name) {
            Some(client) => client.bounds != bounds,
            None => true,
        } {
            for x in bounds.min.x..=bounds.max.x {
                for y in bounds.min.y..=bounds.max.y {
                    let pos = IVec2::new(x, y);
                    if !self.init_flags.contains(&pos) {
                        // generation rules start

                        let tile = Tile::new(IVec3::new(pos.x, pos.y, 0), "Surface".to_string());
                        tile_service.add_tile(tile);

                        // generation rules end

                        self.init_flags.insert(pos);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::IVec3;

    use crate::{models::IBounds3, tile_service::TileService};

    use super::GenerationService;

    #[test]
    fn set_bounds() {
        let mut tile_service = TileService::new();
        let mut gen_service = GenerationService::new();

        let bounds = IBounds3::new(IVec3::new(0, 0, 0), IVec3::new(8, 8, 8));
        gen_service.set_bounds("TEST_CLIENT_NAME".to_string(), bounds, &mut tile_service);
    }
}
