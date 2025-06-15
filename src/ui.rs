use crate::colors::{GREEN, ORANGE, WHITE};
use crate::{
    constants::DISPLAY_DESPAWN_TIME,
    game_state::{EndState, GameEndEvent, GameTime},
    points::Points,
};
use bevy::prelude::{Display, Node, Text, TextColor, TextFont, Without};
use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res},
    },
    prelude::{Deref, DerefMut},
    time::{Time, Timer, TimerMode},
    ui::{AlignItems, JustifyContent, PositionType, UiRect, Val},
};

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct PointDisplayRoot;

#[derive(Component)]
struct PointsText1;

#[derive(Component)]
struct PointsText2;

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct TimeDisplayRoot;

fn setup_time_ui(mut commands: Commands) {
    commands.spawn((
        TimeDisplayRoot,
        Node {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            right: Val::Percent(50.),
            top: Val::Percent(10.),
            padding: UiRect::all(Val::Px(4.0)),
            ..Default::default()
        },
        TextFont {
            font_size: 32.0,
            ..Default::default()
        },
        TextColor(GREEN),
        TimeText,
        Text("Time Left: N/A".into()),
    ));
}

#[derive(Component)]
struct MainUi;

#[derive(Component, Deref, DerefMut)]
struct DisplayTime(pub Timer);

fn spawn_game_end_notification(
    mut commands: Commands,
    mut game_end_event: EventReader<GameEndEvent>,
) {
    for ev in game_end_event.read() {
        let text = match ev.end_state {
            EndState::Player1Won => "Player 1 Won!",
            EndState::Player2Won => "Player 2 Won!",
            EndState::Draw => "Draw :/",
        }
        .to_string();

        commands.spawn((
            MainUi,
            DisplayTime(Timer::from_seconds(DISPLAY_DESPAWN_TIME, TimerMode::Once)),
            (Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(Val::Px(4.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            }),
            (
                Text(text),
                TextColor(WHITE),
                TextFont {
                    font_size: 32.0,
                    ..Default::default()
                },
            ),
        ));
    }
}

fn update_display_timers(time: Res<Time>, mut q_timers: Query<&mut DisplayTime>) {
    for mut display_time in q_timers.iter_mut() {
        display_time.tick(time.delta());
    }
}

fn despawn_entities_with_display_time(
    mut commands: Commands,
    mut notifications: Query<(Entity, &DisplayTime), With<MainUi>>,
) {
    for (entity, display_time) in notifications.iter_mut() {
        if display_time.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn setup_points_ui(mut commands: Commands) {
    commands.spawn((
        PointDisplayRoot,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(45.),
            bottom: Val::Percent(5.0),
            padding: UiRect::all(Val::Px(4.0)),
            ..Default::default()
        },
        PointsText1,
        Text("Player 1".into()),
        TextFont {
            font_size: 32.0,
            ..Default::default()
        },
        TextColor(GREEN),
    ));
    commands.spawn((
        PointDisplayRoot,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Percent(45.),
            bottom: Val::Percent(5.0),
            padding: UiRect::all(Val::Px(4.0)),
            ..Default::default()
        },
        PointsText2,
        Text("Player 2".into()),
        TextFont {
            font_size: 32.0,
            ..Default::default()
        },
        TextColor(ORANGE),
    ));
}

fn point_text_update_system(
    q_points: Query<&Points>,
    mut q_p1: Query<&mut Text, (With<PointsText1>, Without<PointsText2>)>,
    mut q_p2: Query<&mut Text, (With<PointsText2>, Without<PointsText1>)>,
) {
    let points = q_points.single().unwrap();
    if let Ok(mut text) =q_p1.single_mut() {
        text.0 = format!("{}", points.player_1);
    }
    if let Ok(mut text) =q_p2.single_mut() {
        text.0 = format!("{}", points.player_2);
    }
}

fn display_game_time(q_timer: Query<&mut GameTime>, mut query: Query<&mut Text, With<TimeText>>) {
    if let Ok(game_timer) = q_timer.single() {
        for mut text in &mut query {
            text.0 = format!("{}", game_timer.current_time().as_secs());
        }
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (setup_points_ui, setup_time_ui))
            .add_systems(
                Update,
                (
                    point_text_update_system,
                    display_game_time,
                    spawn_game_end_notification,
                    update_display_timers,
                    despawn_entities_with_display_time,
                ),
            );
    }
}
