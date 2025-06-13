use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        event::EventReader,
        system::{Commands, Query},
    },
};

use crate::goals::{GoalEvent, PlayerType};

fn update_player_points(mut q_points: Query<&mut Points>, mut goal_events: EventReader<GoalEvent>) {
    let mut points = q_points.single_mut().unwrap();
    for ev in goal_events.read() {
        match ev.player {
            PlayerType::First => points.player_1 += ev.amount,
            PlayerType::Second => points.player_2 += ev.amount,
        }
    }
}

#[derive(Component, Default)]
pub struct Points {
    pub player_1: u32,
    pub player_2: u32,
}

fn spawn_points(mut commands: Commands) {
    commands.spawn(Points::default());
}

pub struct PointsPlugin;
impl Plugin for PointsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_points)
            .add_systems(Update, (update_player_points,));
    }
}
