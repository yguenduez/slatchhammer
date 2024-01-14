use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::{Commands, Query},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::{Text, TextSection, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        BackgroundColor, PositionType, Style, UiRect, Val, ZIndex,
    },
};

use crate::goals::{GoalEvent, PlayerType};

fn update_player_points(mut q_points: Query<&mut Points>, mut goal_events: EventReader<GoalEvent>) {
    let mut points = q_points.single_mut();
    for ev in goal_events.read() {
        match ev.player {
            PlayerType::First => points.player_1 += ev.amount,
            PlayerType::Second => points.player_2 += ev.amount,
        }
    }
}

#[derive(Component, Default)]
pub struct Points {
    player_1: u32,
    player_2: u32,
}

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct PointDisplayRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct PointsText;

fn setup_points_ui(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            PointDisplayRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
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
    // create our text
    let text_style = TextStyle {
        font_size: 16.,
        color: Color::WHITE,
        ..Default::default()
    };
    let text_fps = commands
        .spawn((
            PointsText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "Player 1: ".into(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "----".into(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "Player 2:".into(),
                        style: text_style,
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
        text.sections[0].value = format!("Player 1:{}", points.player_1);
        text.sections[2].value = format!("Player 2:{}", points.player_2);
    }
}

fn spawn_points(mut commands: Commands) {
    commands.spawn(Points::default());
}

pub struct PointsPlugin;
impl Plugin for PointsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (setup_points_ui, spawn_points))
            .add_systems(Update, (update_player_points, point_text_update_system));
    }
}
