use bevy::{math::bounding::*, color::palettes::css::{BLUE, RED}, prelude::*};
use rand::Rng;

use crate::{bounding::{Intersects, Shape, Volume}, bullet::Bullet, player::{Player, PlayerHitEvent, PLAYER_SIZE}, AppState};

const SPAWN_DELAY: f32 = 1.;
const DEATH_TIME: f32 = 0.5;
const SPEED: f32 = 30.;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_enemy.run_if(time_passed(SPAWN_DELAY)),
            move_enemies,
            (
                (enemy_bullet_collision, enemy_player_collision),
                play_death,
                despawn_dead
            ).chain()
        ).run_if(in_state(AppState::InGame)))
        .add_event::<EnemyKilledEvent>();
    }
}

#[derive(Event, Default)]
pub struct EnemyKilledEvent;

fn time_passed(t: f32) -> impl FnMut(Local<f32>, Res<Time>) -> bool {
    move |mut timer: Local<f32>, time: Res<Time>| {
        // Tick the timer
        *timer += time.delta_seconds();
        // Return true if the timer has passed the time
        *timer >= t
    }
}

#[derive(Component)]
pub struct Enemy {
    death_material: Handle<ColorMaterial>
}

#[derive(Component, Default, Debug)]
pub struct Health(pub i8);

#[derive(Component, Default)]
pub struct Death(f32);

impl Health {
    pub fn hit(&mut self, hit_points: i8) {
        self.0 -= hit_points;
    }
}

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

    let triangle_primitive = Triangle2d::new(
        Vec2::Y * 20.0,
        Vec2::new(-20.0, -20.0),
        Vec2::new(20.0, -20.0),
    );

    commands.spawn((ColorMesh2dBundle {
        mesh: meshes.add(triangle_primitive).into(),
        material: materials.add(Color::from(BLUE)),
        transform: Transform::from_translation(position),
        ..default()
    }, Enemy {
        death_material: materials.add(Color::from(RED)),
    }, Health(1), Shape::Triangle(triangle_primitive), Intersects::default()));
}

pub fn play_death(
    mut commands: Commands, 
    mut enemies_query: Query<(Entity, &Health, &Enemy), Without<Death>>,
    time: Res<Time>
) {
    for (enemy_entity, enemy_health, enemy) in &mut enemies_query {
        if enemy_health.0 <= 0 {
            commands.entity(enemy_entity).insert((
                enemy.death_material.clone(),
                
                Death(time.elapsed_seconds())
            ));
        }
    }
}

pub fn despawn_dead(
    mut commands: Commands,
    mut enemies_query: Query<(Entity, &Death), With<Enemy>>,
    time: Res<Time>
) {
    for (enemy_entity, enemy_death) in &mut enemies_query {
        if time.elapsed_seconds() - enemy_death.0 > DEATH_TIME {
            commands.entity(enemy_entity).despawn();
        } 
    }
}

pub fn move_enemies(
    mut enemies_query: Query<(Entity, &mut Transform), (With<Enemy>, Without<Death>, Without<Player>)>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    time: Res<Time>
) {
    let Ok((player_entity, player_transform)) = player_query.get_single_mut() else {
        return;
    };

    for (enemy_entity, mut enemy_transform) in &mut enemies_query {
        let mut dir = player_transform.translation - enemy_transform.translation;
        dir = dir.normalize();
        
        enemy_transform.translation += dir * time.delta_seconds() * SPEED;
    }
}

pub fn enemy_bullet_collision(
    mut commands: Commands,
    bullets_query: Query<(Entity, &Volume), With<Bullet>>,
    mut enemies_query: Query<(&mut Health, &Volume), (With<Enemy>, Without<Player>, Without<Death>)>,
    mut player_killed_events: EventWriter<EnemyKilledEvent>
) {
    for (mut enemy_health, enemy_volume) in &mut enemies_query {
        let mut has_intersected = false;
        for (bullet_entity, bullet_volume) in &bullets_query {
            if enemy_volume.intersects(&bullet_volume.0) {
                enemy_health.hit(1);
                commands.entity(bullet_entity).despawn();
                has_intersected = true;
                player_killed_events.send_default();
                break;
            }
        }
        if has_intersected {
            break;
        }
    }
}

pub fn enemy_player_collision(
    mut enemies_query: Query<(&mut Health, &Volume), (With<Enemy>, Without<Player>, Without<Death>)>,
    mut player_query: Query<(&mut Health, &Volume), With<Player>>,
    mut player_hit_events: EventWriter<PlayerHitEvent>
) {

    let Ok((mut player_health, player_volume)) = player_query.get_single_mut() else {
        return;
    };

    for (mut enemy_health, enemy_volume) in &mut enemies_query {
        if player_volume.intersects(&enemy_volume.0) {
            player_health.hit(1);
            enemy_health.hit(1);
            player_hit_events.send_default();
            break;
        }
    }
}