
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub const PLAYER_SIZE: f32 = 64.;
const SPEED: f32 = 200.;
const JUMP_SPEED: f32 = 0.5;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player {
    is_jumping: bool
}

#[derive(Resource, Default)]
pub struct PlayerAnimations {
    jump: Option<Handle<AnimationClip>>
}

#[derive(Event, Default)]
pub struct PlayerJumpStartEvent;

#[derive(Event, Default)]
pub struct PlayerJumpEndEvent;

#[derive(Event, Default)]
pub struct PlayerMoveEvent(pub Vec3);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PlayerAnimations>()
        .add_event::<PlayerJumpStartEvent>()
        .add_event::<PlayerJumpEndEvent>()
        .add_event::<PlayerMoveEvent>()
        .add_systems(Startup, (spawn_player, generate_jump_animation).chain())
        .add_systems(Startup, generate_jump_animation)
        .add_systems(Update, (move_player, jump_player, change_color))
        ;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_jump = Name::new("player_jump");

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(PLAYER_SIZE)),
        material: materials.add(Color::PURPLE),
        ..default()
    }, Player::default(), AnimationPlayer::default(), player_jump));
}

fn generate_jump_animation(
    mut animations: ResMut<Assets<AnimationClip>>,
    mut player_animations: ResMut<PlayerAnimations>,
    mut player_name_query: Query<&Name, With<Player>>,
) {

    let Ok(player_name) = player_name_query.get_single_mut() else {
        return;
    };
    let mut animation = AnimationClip::default();

    animation.add_curve_to_path(
        EntityPath {
            parts: vec![player_name.clone()]
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, JUMP_SPEED / 2., JUMP_SPEED, JUMP_SPEED + JUMP_SPEED / 2., 2. * JUMP_SPEED],
            keyframes: Keyframes::Scale(vec![
                Vec3::splat(PLAYER_SIZE),
                Vec3::splat(PLAYER_SIZE + 0.25 * PLAYER_SIZE),
                Vec3::splat(PLAYER_SIZE + 0.5 * PLAYER_SIZE),
                Vec3::splat(PLAYER_SIZE + 0.25 * PLAYER_SIZE),
                Vec3::splat(PLAYER_SIZE),
            ]),
            interpolation: Interpolation::Linear,
        },
    );

    player_animations.jump = Some(animations.add(animation))
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
    time: Res<Time>
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let movement = direction.normalize_or_zero() * time.delta_seconds() * SPEED;

    player_transform.translation += movement;
    if movement != Vec3::ZERO {
        player_move_events.send(PlayerMoveEvent(direction));
    }
}
    
fn jump_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut AnimationPlayer, &mut Player)>,
    player_animations: Res<PlayerAnimations>,
    mut player_jump_start_events: EventWriter<PlayerJumpStartEvent>,
    mut player_jump_end_events: EventWriter<PlayerJumpEndEvent>,
) {
    let Ok((mut player_animation_player, mut player)) = player_query.get_single_mut() else {
        return;
    };

    let Some(jump_animation) = player_animations.jump.clone() else {
        return;
    };
    
    if player_animation_player.is_finished() {
        player.is_jumping = false;
        if !player_animation_player.is_paused() {
            player_jump_end_events.send_default();
            player_animation_player.pause();
        }
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        if player_animation_player.is_finished() {
            player_animation_player.replay();
            player_animation_player.resume();
        }
        player_animation_player.play(jump_animation);
        player.is_jumping = true;
        player_jump_start_events.send_default();
    }
}

pub fn change_color(
    mut player_jump_start_events: EventReader<PlayerJumpStartEvent>,
    mut player_jump_end_events: EventReader<PlayerJumpEndEvent>,
    player_query: Query<&Handle<ColorMaterial>, With<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok(player_material_handle) = player_query.get_single() else {
        return;
    };

    let Some(player_material) = materials.get_mut(player_material_handle) else {
        return;
    };

    for _ in player_jump_start_events.read() {
        player_material.color = Color::YELLOW.into();
    }

    for _ in player_jump_end_events.read() {
        player_material.color = Color::PURPLE;
    }
}