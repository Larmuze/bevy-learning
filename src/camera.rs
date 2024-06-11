use bevy::prelude::*;

use crate::player::Player;

const DIR_LERP: f32 = 0.02;

pub struct CameraPlugin;

impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, move_camera)
        ;
    }
}

pub fn setup_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

pub fn move_camera(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    let direction = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        camera_transform.translation.z
    );

    camera_transform.translation = camera_transform.translation.lerp(direction, DIR_LERP);
}