use bevy::prelude::*;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup() {}
