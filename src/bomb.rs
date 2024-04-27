use bevy::prelude::*;

const BOMB_TIME: f32 = 2.0;

const BOMB_SIZE: f32 = 15.0;
const BOMB_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub struct Bomb {
    timer: Timer,  
}

#[derive(Bundle)]
pub struct BombBundle {
    bomb: Bomb,
    sprite: SpriteBundle,
}

impl BombBundle {
    pub fn new(translation: Vec3) -> Self {
        // TODO: see if we can make this circles again using meshes, but still practical
        BombBundle {
            bomb: Bomb { timer: Timer::from_seconds(BOMB_TIME, TimerMode::Once) },
            sprite: SpriteBundle {
                transform: Transform {
                    translation,
                    scale: Vec2::splat(BOMB_SIZE).extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: BOMB_COLOR,
                    ..default()
                },
                ..default()
            },
        }
    } 
}

pub fn explode(
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

