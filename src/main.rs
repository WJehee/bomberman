use bevy::prelude::*;

mod player;
mod obstacle;

use player::PlayerPlugin;
use obstacle::ObstaclePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, ObstaclePlugin))
        .add_systems(Startup, hello)
        .run();
}

fn hello(
    mut commands: Commands,
) {
    println!("Hello world!");
    commands.spawn(Camera2dBundle::default());
}

