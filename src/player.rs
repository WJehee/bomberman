use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const PLAYER_SPEED: f32 = 500.0;

const PLAYER_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);
const PLAYER_START: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const PLAYER_SIZE: f32 = 30.0;

const BOMB_SIZE: f32 = 15.0;
const BOMB_TIME: f32 = 2.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_player)
            .add_systems(FixedUpdate, (move_player, explode).chain());
    }
}

#[derive(Component)]
struct Player;

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
                .with_scale(Vec2::splat(PLAYER_SIZE).extend(1.)),
            ..default()
        },
        Player,
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    let mut player_transform = query.single_mut();
    let mut x_dir = 0.0;
    let mut y_dir = 0.0;

    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::KeyW  => { y_dir += 1.0 },
            KeyCode::KeyS  => { y_dir -= 1.0 },
            KeyCode::KeyD  => { x_dir += 1.0 },
            KeyCode::KeyA  => { x_dir -= 1.0 },
            KeyCode::Space => {
                commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Circle::default()).into(),
                            material: materials.add(PLAYER_COLOR),
                            transform: Transform::from_translation(player_transform.translation)
                                .with_scale(Vec2::splat(BOMB_SIZE).extend(1.)),
                                ..default()
                        },
                        Bomb { timer: Timer::from_seconds(BOMB_TIME, TimerMode::Once) },
                ));
            },
            _ => {},
        } 
    }
    player_transform.translation.x = player_transform.translation.x + x_dir * PLAYER_SPEED * time.delta_seconds();
    player_transform.translation.y = player_transform.translation.y + y_dir * PLAYER_SPEED * time.delta_seconds();
}

#[derive(Component)]
pub struct Bomb {
    timer: Timer,  
}

fn explode(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Bomb)>,
    time: Res<Time>,
    ) {
    for (entity, mut bomb) in &mut query {
        bomb.timer.tick(time.delta());
        if bomb.timer.finished() {
            commands.entity(entity).despawn();
        } 
    } 
}

