use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const Z_INDEX: f32 = 2.0;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Cuboid::default()).into(),
            material: materials.add(COLOR),
            transform: Transform {
                translation: Vec2::new(-200.0, 0.0).extend(Z_INDEX),
                scale: Vec2::new(100.0, 200.0).extend(0.0),
                ..default()
            },
            ..default()
        },
    ));
}


