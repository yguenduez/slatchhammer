use bevy::{
    app::{Plugin, Update},
    ecs::{event::EventReader, query::With, system::Query},
    transform::components::Transform,
};
use bevy_rapier3d::dynamics::Velocity;

use crate::{
    constants::{
        BALL_STARTING_POINT, BALL_STARTING_VELOCITY, PLAYER1_STARTING_POINT, PLAYER2_STARTING_POINT,
    },
    goals::GoalEvent,
    player::{Player1, Player2},
    Ball,
};

fn reset_p1_after_goal(
    mut goal_event: EventReader<GoalEvent>,
    mut q: Query<&mut Transform, With<Player1>>,
) {
    for _ in goal_event.read() {
        let mut t = q.single_mut();
        t.translation = PLAYER1_STARTING_POINT;
    }
}
fn reset_p2_after_goal(
    mut goal_event: EventReader<GoalEvent>,
    mut q: Query<&mut Transform, With<Player2>>,
) {
    for _ in goal_event.read() {
        let mut t = q.single_mut();
        t.translation = PLAYER2_STARTING_POINT;
    }
}
fn reset_ball_after_goal(
    mut goal_event: EventReader<GoalEvent>,
    mut q: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    for _ in goal_event.read() {
        let (mut t, mut v) = q.single_mut();
        t.translation = BALL_STARTING_POINT;
        v.linvel = BALL_STARTING_VELOCITY;
    }
}

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                reset_p1_after_goal,
                reset_p2_after_goal,
                reset_ball_after_goal,
            ),
        );
    }
}
