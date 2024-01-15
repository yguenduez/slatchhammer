use std::time::Duration;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res},
    },
    time::{Time, Timer, TimerMode},
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

#[derive(Component)]
pub struct GameTime {
    time: Timer,
}

impl GameTime {
    pub fn current_time(&self) -> Duration {
        self.time.duration() - self.time.elapsed()
    }
}

fn update_game_timer(time: Res<Time>, mut q_time: Query<&mut GameTime>) {
    if let Ok(mut timer) = q_time.get_single_mut() {
        timer.time.tick(time.delta());
    }
}

fn spawn_game_timer(mut commands: Commands) {
    commands.spawn(GameTime {
        time: Timer::new(Duration::from_secs(180), TimerMode::Once),
    });
}

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_game_timer).add_systems(
            Update,
            (
                reset_p1_after_goal,
                reset_p2_after_goal,
                reset_ball_after_goal,
                update_game_timer,
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::time::{Timer, TimerMode};

    use super::GameTime;

    #[test]
    fn current_time_when_called_return_duration() {
        // given
        let time = GameTime {
            time: Timer::new(Duration::from_secs(2), TimerMode::Once),
        };

        // when
        let dur = time.current_time();

        // then
        assert_eq!(dur, Duration::from_secs(2));
    }
}
