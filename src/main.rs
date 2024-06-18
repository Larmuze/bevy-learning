use bevy::prelude::*;
use bounding::BoundingPlugin;
use bullet::BulletPlugin;
use camera::CameraPlugin;
use death::DeathPlugin;
use enemies::EnemyPlugin;
use game::GamePlugin;
use level::LevelPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

mod player;
mod camera;
mod level;
mod bullet;
mod enemies;
mod bounding;
mod menu;
mod death;
mod game;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    EndGame,
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(BoundingPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(DeathPlugin)
        .add_plugins(GamePlugin)
        .run();
}
