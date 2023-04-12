use std::collections::HashMap;

use glam::IVec3;

use crate::models::{IBounds3, Tile};

#[derive(Debug, Clone)]
pub enum TileCmd {
    Add(Tile),
    Remove(IVec3),
}

pub struct TileClient {
    bounds: IBounds3,
    cmds: Vec<TileCmd>,
}

impl TileClient {
    pub fn new(bounds: IBounds3, cmds: Vec<TileCmd>) -> Self {
        Self { bounds, cmds }
    }
}

pub struct TileService {
    tiles: HashMap<IVec3, Tile>,
    clients: HashMap<String, TileClient>,
}

impl TileService {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            clients: HashMap::new(),
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        if self.tiles.contains_key(&tile.pos) {
            panic!("tile is already existed at pos {:?}.", tile.pos);
        }
        self.tiles.insert(tile.pos.clone(), tile.clone());

        for (_, client) in &mut self.clients {
            if client.bounds.inclusive_contains(&tile.pos) {
                client.cmds.push(TileCmd::Add(tile.clone()));
            }
        }
    }

    pub fn remove_tile(&mut self, pos: IVec3) {
        if !self.tiles.contains_key(&pos) {
            panic!("tile is not found at pos {:?}", pos);
        }
        self.tiles.remove(&pos);

        for (_, client) in &mut self.clients {
            if client.bounds.inclusive_contains(&pos) {
                client.cmds.push(TileCmd::Remove(pos.clone()));
            }
        }
    }

    pub fn get_tile(&self, pos: IVec3) -> Option<Tile> {
        self.tiles.get(&pos).cloned()
    }

    pub fn set_bounds(&mut self, client_name: String, bounds: IBounds3) {
        if let Some(client) = self.clients.get_mut(&client_name) {
            for x in client.bounds.min.x..=client.bounds.max.x {
                for y in client.bounds.min.y..=client.bounds.max.y {
                    for z in client.bounds.min.z..=client.bounds.max.z {
                        let pos = IVec3::new(x, y, z);
                        if !bounds.inclusive_contains(&pos) && self.tiles.contains_key(&pos) {
                            client.cmds.push(TileCmd::Remove(pos));
                        }
                    }
                }
            }

            for x in bounds.min.x..=bounds.max.x {
                for y in bounds.min.y..=bounds.max.y {
                    for z in bounds.min.z..=bounds.max.z {
                        let pos = IVec3::new(x, y, z);
                        if !client.bounds.inclusive_contains(&pos) && self.tiles.contains_key(&pos)
                        {
                            let tile = self.tiles.get(&pos).unwrap();
                            client.cmds.push(TileCmd::Add(tile.clone()));
                        }
                    }
                }
            }

            client.bounds = bounds;
        } else {
            let mut cmds = vec![];

            for x in bounds.min.x..=bounds.max.x {
                for y in bounds.min.y..=bounds.max.y {
                    for z in bounds.min.z..=bounds.max.z {
                        let pos = IVec3::new(x, y, z);
                        if self.tiles.contains_key(&pos) {
                            let tile = self.tiles.get(&pos).unwrap();
                            cmds.push(TileCmd::Add(tile.clone()));
                        }
                    }
                }
            }

            let client = TileClient::new(bounds, cmds);
            self.clients.insert(client_name, client);
        }
    }

    pub fn get_commands(&mut self, client_name: String) -> Vec<TileCmd> {
        let Some(client) = self.clients.get_mut(&client_name) else {
            panic!("client named {:?} is not found", client_name);
        };

        let mut out_cmds = vec![];
        std::mem::swap(&mut client.cmds, &mut out_cmds);
        out_cmds
    }
}
