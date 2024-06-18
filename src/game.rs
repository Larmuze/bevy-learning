use bevy:: prelude::*;

use crate::{enemies::EnemyKilledEvent, player::{PlayerHitEvent, PLAYER_LIFES}, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup_ui)
            .add_systems(Update, (update_score, update_health_ui).run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), cleanup_menu)
        ;
    }
}

#[derive(Resource)]
struct MenuData {
    score_entity: Entity,
}

#[derive(Component, Default)]
struct Score(pub i32);

#[derive(Component, Default)]
struct Health_UI(pub i8);

fn setup_ui(
    mut commands: Commands
) {
    let score_entity = commands
    .spawn(NodeBundle {
        style: Style {
            // center button
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn((TextBundle {
                text: Text::from_section("0", TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                }), 
                style: Style {
                    width: Val::Percent(50.),
                    height: Val::Px(65.),
                    ..default()
                },
                ..default()
            }, Score::default()));

            parent
            .spawn((TextBundle {
                text: Text::from_section(PLAYER_LIFES.to_string(), TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
            
                    ..default()
                }), 
                style: Style {
                    width: Val::Px(100.),
                    height: Val::Px(65.),
                    ..default()
                },
                ..default()
            }, Health_UI(PLAYER_LIFES)));
    })
    .id();
commands.insert_resource(MenuData { score_entity });
}


fn update_score(
    mut events: EventReader<EnemyKilledEvent>,
    mut text_query: Query<(&mut Text, &mut Score)>
) {
    let Ok((mut text, mut score)) = text_query.get_single_mut() else {
        return;
    };

    for _ in events.read() {
        score.0 += 1;
        text.sections[0].value = score.0.to_string();
    }
}

fn update_health_ui(
    mut events: EventReader<PlayerHitEvent>,
    mut text_query: Query<(&mut Text, &mut Health_UI)>,
) {
    let Ok((mut text, mut health_ui)) = text_query.get_single_mut() else {
        return;
    };

    for _ in events.read() {
        health_ui.0 -= 1;
        text.sections[0].value = health_ui.0.to_string();
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.score_entity).despawn_recursive();
}