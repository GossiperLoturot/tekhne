use std::collections::HashMap;

use crate::models::{Bounds3D, Pos3D, Tile};

#[derive(Debug, Clone)]
pub enum TileCommand {
    AddTile(Tile),
    RemoveTile(Pos3D<i32>),
}

pub struct TileService {
    tiles: HashMap<Pos3D<i32>, Tile>,
    clients: HashMap<String, Bounds3D<i32>>,
    commands: HashMap<String, Vec<TileCommand>>,
}

impl TileService {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            clients: HashMap::new(),
            commands: HashMap::new(),
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        if self.tiles.contains_key(&tile.pos) {
            panic!("tile is already existed at pos {:?}.", tile.pos);
        }
        self.tiles.insert(tile.pos.clone(), tile.clone());

        for (client_name, bounds) in &self.clients {
            if bounds.inclusive_contains(&tile.pos) {
                let cmd = self.commands.get_mut(client_name).unwrap();
                cmd.push(TileCommand::AddTile(tile.clone()));
            }
        }
    }

    pub fn remove_tile(&mut self, pos: Pos3D<i32>) {
        if !self.tiles.contains_key(&pos) {
            panic!("tile is not found at pos {:?}", pos);
        }
        self.tiles.remove(&pos);

        for (client_name, bounds) in &self.clients {
            if bounds.inclusive_contains(&pos) {
                let cmd = self.commands.get_mut(client_name).unwrap();
                cmd.push(TileCommand::RemoveTile(pos.clone()));
            }
        }
    }

    pub fn get_tile(&self, pos: Pos3D<i32>) -> Option<Tile> {
        self.tiles.get(&pos).cloned()
    }

    pub fn set_bounds(&mut self, client_name: String, bounds: Bounds3D<i32>) {
        self.clients.insert(client_name.clone(), bounds);

        if !self.commands.contains_key(&client_name) {
            self.commands.insert(client_name, vec![]);
        }
    }

    pub fn get_commands(&mut self, client_name: String) -> Vec<TileCommand> {
        let Some(cmd) = self.commands.get_mut(&client_name) else {
            panic!("client named {:?} is not found", client_name);
        };

        let mut res = vec![];
        std::mem::swap(cmd, &mut res);
        res
    }
}
