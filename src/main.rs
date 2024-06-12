use bevy::prelude::*;
use bounding::BoundingPlugin;
use bullet::BulletPlugin;
use camera::CameraPlugin;
use enemies::EnemyPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;

mod player;
mod camera;
mod level;
mod bullet;
mod enemies;
mod bounding;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(BoundingPlugin)
        .run();
}
