mod entity_service;
mod generation_service;
mod models;
mod tile_service;

use bevy::prelude::*;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup() {}
