use bevy::prelude::*;
use rand::Rng;

use crate::player::{Player, PLAYER_SIZE};

const SPAWN_DELAY: f32 = 1.;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemy.run_if(time_passed(SPAWN_DELAY)))
        ;
    }
}

fn time_passed(t: f32) -> impl FnMut(Local<f32>, Res<Time>) -> bool {
    move |mut timer: Local<f32>, time: Res<Time>| {
        // Tick the timer
        *timer += time.delta_seconds();
        // Return true if the timer has passed the time
        *timer >= t
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct Health(i8);

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single_mut() else {
        return;
    };

    let mut rng = rand::thread_rng();
    let mut position = player_transform.translation;
    
    loop {
        position = player_transform.translation + Vec3::new(
           rng.gen_range(-800.0..800.),
           rng.gen_range(-800.0..800.),
           0.,
       );

       if player_transform.translation.distance_squared(position) > 8. * PLAYER_SIZE * PLAYER_SIZE {
            break;
       }
    }
    commands.spawn((ColorMesh2dBundle {
        mesh: meshes.add(Triangle2d::new(
            Vec2::Y * 20.0,
            Vec2::new(-20.0, -20.0),
            Vec2::new(20.0, -20.0),
        )).into(),
        material: materials.add(Color::RED),
        transform: Transform::from_translation(position),
        ..default()
    }, Enemy, Health(1)));
}