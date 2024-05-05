use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::bomb::{BombBundle, explode};

const PLAYER_SPEED: f32 = 500.0;
const BOMB_COOLDOWN: f32 = 0.5;
const PLAYER_START_HEALTH: u8 = 3;

const PLAYER_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const PLAYER_START: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const PLAYER_SIZE: f32 = 30.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let mut timer = Timer::from_seconds(BOMB_COOLDOWN, TimerMode::Once);
        timer.tick(Duration::from_secs_f32(BOMB_COOLDOWN));
        app
            .insert_resource(BombCooldownTimer(timer))
            .add_systems(Startup, init_player)
            .add_systems(FixedUpdate, (player_controls, explode).chain());
    }
}

#[derive(Component)]
struct Player {
    health: u8,
}

#[derive(Resource)]
struct BombCooldownTimer(Timer);

fn init_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(PLAYER_COLOR),
            transform: Transform::from_translation(PLAYER_START)
                .with_scale(Vec2::splat(PLAYER_SIZE).extend(1.0)),
            ..default()
        },
        Player { health: PLAYER_START_HEALTH },
    ));
}

fn player_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cooldown_timer: ResMut<BombCooldownTimer>,
    mut query: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    ) {
    let mut player_transform = query.single_mut();
    let mut x_dir = 0.0;
    let mut y_dir = 0.0;

    cooldown_timer.0.tick(time.delta());

    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::KeyW  => { y_dir += 1.0 },
            KeyCode::KeyS  => { y_dir -= 1.0 },
            KeyCode::KeyD  => { x_dir += 1.0 },
            KeyCode::KeyA  => { x_dir -= 1.0 },
            KeyCode::Space => { 
                if cooldown_timer.0.finished() {
                    commands.spawn(BombBundle::new(player_transform.translation));
                    cooldown_timer.0.reset();
                }
            },
            _ => {},
        } 
    }
    player_transform.translation.x = player_transform.translation.x + x_dir * PLAYER_SPEED * time.delta_seconds();
    player_transform.translation.y = player_transform.translation.y + y_dir * PLAYER_SPEED * time.delta_seconds();
}

