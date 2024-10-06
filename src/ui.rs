use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    prelude::{Deref, DerefMut},
    text::{Text, TextSection, TextStyle},
    time::{Time, Timer, TimerMode},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, BackgroundColor, JustifyContent, PositionType, Style, UiRect, Val, ZIndex,
    },
};

use crate::colors::{BLACK, GREEN, ORANGE, WHITE};
use crate::{
    constants::DISPLAY_DESPAWN_TIME,
    game_state::{EndState, GameEndEvent, GameTime},
    points::Points,
};

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct PointDisplayRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct PointsText;

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct TimeDisplayRoot;

fn setup_time_ui(mut commands: Commands) {
    let root = commands
        .spawn((
            TimeDisplayRoot,
            NodeBundle {
                background_color: BackgroundColor(BLACK),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(45.),
                    top: Val::Percent(1.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let color = TextStyle {
        font_size: 16.,
        color: GREEN,
        ..Default::default()
    };

    let time_text = commands
        .spawn((
            TimeText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "Time Left: ".into(),
                        style: color.clone(),
                    },
                    TextSection {
                        value: "N/A".into(),
                        style: color,
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[time_text]);
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
        println!("I GOT SPAWNED!");
        let color = TextStyle {
            font_size: 32.,
            color: WHITE,
            ..Default::default()
        };

        let text = match ev.end_state {
            EndState::Player1Won => "Player 1 Won!",
            EndState::Player2Won => "Player 2 Won!",
            EndState::Draw => "Draw :/",
        }
        .to_string();

        commands
            .spawn((
                MainUi,
                DisplayTime(Timer::from_seconds(DISPLAY_DESPAWN_TIME, TimerMode::Once)),
                NodeBundle {
                    background_color: BackgroundColor(BLACK),
                    z_index: ZIndex::Global(i32::MAX),
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        padding: UiRect::all(Val::Px(4.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((TextBundle {
                    text: Text::from_sections([TextSection {
                        value: text,
                        style: color,
                    }]),
                    ..Default::default()
                },));
            });
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
            println!("I WAS TRIGGERED!");
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn setup_points_ui(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            PointDisplayRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(BLACK),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(45.),
                    top: Val::Percent(5.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let colors: Vec<TextStyle> = [GREEN, WHITE, ORANGE]
        .iter()
        .map(|c| TextStyle {
            font_size: 16.,
            color: *c,
            ..Default::default()
        })
        .collect();

    let text_fps = commands
        .spawn((
            PointsText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "Player 1: ".into(),
                        style: colors[0].clone(),
                    },
                    TextSection {
                        value: ":".into(),
                        style: colors[1].clone(),
                    },
                    TextSection {
                        value: "Player 2:".into(),
                        style: colors[2].clone(),
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

fn point_text_update_system(
    q_points: Query<&Points>,
    mut query: Query<&mut Text, With<PointsText>>,
) {
    let points = q_points.single();
    for mut text in &mut query {
        text.sections[0].value = format!("{}", points.player_1);
        text.sections[2].value = format!("{}", points.player_2);
    }
}

fn display_game_time(q_timer: Query<&mut GameTime>, mut query: Query<&mut Text, With<TimeText>>) {
    if let Ok(game_timer) = q_timer.get_single() {
        for mut text in &mut query {
            text.sections[1].value = format!("{}", game_timer.current_time().as_secs());
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
