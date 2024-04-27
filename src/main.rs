use bevy::prelude::*;

mod player;

use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(Startup, hello)
        .run();
}

fn hello(
    mut commands: Commands,
) {
    println!("Hello world!");
    commands.spawn(Camera2dBundle::default());
}

