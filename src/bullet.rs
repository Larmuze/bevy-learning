use bevy::prelude::*;
use rand::{random, Rng};

use crate::{bounding::{Intersects, Shape}, player::{Player, PlayerMoveEvent}};

const BULLET_SPEED: f32 = 300.;
const BULLET_LIFETIME: f32 = 2.;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, spawn_bullets)
        .add_systems(Update, move_bullets)
        .add_systems(PostUpdate, despawn_bullet)
        ;
    }
}

#[derive(Component)]
pub struct Bullet {
    direction: Vec3,
    spawn_time: f32
}

pub fn spawn_bullets(
    mut commands: Commands,
    mut player_move_events: EventReader<PlayerMoveEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let Some(move_event) = player_move_events.read().next() else {
        return;
    };

    let mut color = Color::srgb(random(), random(), random());
    let mut rng = rand::thread_rng();
    let direction = move_event.0.clone() + Vec3::new(
        rng.gen_range(-0.4..0.4),
        rng.gen_range(-0.4..0.4),
        0.
    );

    let ellipse_primitive = Ellipse::new(3.0, 6.0);
    commands.spawn((ColorMesh2dBundle {
        mesh: meshes.add(ellipse_primitive).into(),
        material: materials.add(color),
        transform: Transform::from_translation(player_transform.translation),
        ..default()
    }, Bullet {
        direction: direction.normalize(),
        spawn_time: time.elapsed_seconds()
    }, Shape::Ellipse(ellipse_primitive),  Intersects::default()));
}

pub fn move_bullets(
    mut bullet_query: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>
) {
    for (mut transform, bullet) in &mut bullet_query {
        transform.translation += bullet.direction * time.delta_seconds() * BULLET_SPEED;
    }

    // println!("Bullet counter : {:?}", bullet_query.iter().len())
}

pub fn despawn_bullet(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Bullet)>,
    time: Res<Time>
) {
    for (entity, bullet) in &mut bullet_query {
        if time.elapsed_seconds() - bullet.spawn_time > BULLET_LIFETIME {
            commands.entity(entity).despawn();
        }
    }
}