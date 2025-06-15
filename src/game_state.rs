use std::time::Duration;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        event::{Event, EventReader, EventWriter},
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
};
use bevy_rapier3d::dynamics::Velocity;

use crate::{
    constants::{
        BALL_STARTING_POINT, BALL_STARTING_VELOCITY, GAME_TIME, PLAYER1_STARTING_POINT,
        PLAYER2_STARTING_POINT,
    },
    goals::GoalEvent,
    player::{Player1, Player2},
    points::Points,
    sprint::SprintState,
    Ball,
};

fn reset_p1_after_goal(
    mut goal_event: EventReader<GoalEvent>,
    mut q: Query<&mut Transform, With<Player1>>,
) {
    for _ in goal_event.read() {
        let mut t = q.single_mut().unwrap();
        t.translation = PLAYER1_STARTING_POINT;
    }
}
fn reset_p2_after_goal(
    mut goal_event: EventReader<GoalEvent>,
    mut q: Query<&mut Transform, With<Player2>>,
) {
    for _ in goal_event.read() {
        let mut t = q.single_mut().unwrap();
        t.translation = PLAYER2_STARTING_POINT;
    }
}
fn reset_ball_after_goal(
    mut goal_event: EventReader<GoalEvent>,
    mut q: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    for _ in goal_event.read() {
        let (mut t, mut v) = q.single_mut().unwrap();
        t.translation = BALL_STARTING_POINT;
        v.linvel = BALL_STARTING_VELOCITY;
    }
}

#[derive(Component)]
pub struct GameTime {
    time: Timer,
}

fn reset_game(
    mut q_time: Query<&mut GameTime>,
    mut q_points: Query<&mut Points>,
    mut game_end_event: EventReader<GameEndEvent>,
    mut q_p1: Query<
        (&mut Transform, &mut SprintState),
        (With<Player1>, Without<Player2>, Without<Ball>),
    >,
    mut q_p2: Query<
        (&mut Transform, &mut SprintState),
        (With<Player2>, Without<Player1>, Without<Ball>),
    >,
    mut q_ball: Query<
        (&mut Transform, &mut Velocity),
        (With<Ball>, Without<Player2>, Without<Player1>),
    >,
) {
    for _ in game_end_event.read() {
        if let Ok(mut timer) = q_time.single_mut() {
            timer.time.reset();
        }

        let mut points = q_points.single_mut().unwrap();
        points.player_1 = 0;
        points.player_2 = 0;
        let (mut t_p1, mut sprint_p1) = q_p1.single_mut().unwrap();
        t_p1.translation = PLAYER1_STARTING_POINT;
        sprint_p1.reset();
        let (mut t_p2, mut sprint_p2) = q_p2.single_mut().unwrap();
        t_p2.translation = PLAYER2_STARTING_POINT;
        sprint_p2.reset();
        let (mut t, mut v) = q_ball.single_mut().unwrap();
        t.translation = BALL_STARTING_POINT;
        v.linvel = BALL_STARTING_VELOCITY;
    }
}

impl GameTime {
    pub fn current_time(&self) -> Duration {
        self.time.duration() - self.time.elapsed()
    }

    pub fn just_finished(&self) -> bool {
        self.time.just_finished()
    }
}

fn update_game_timer(time: Res<Time>, mut q_time: Query<&mut GameTime>) {
    if let Ok(mut timer) = q_time.single_mut() {
        timer.time.tick(time.delta());
    }
}

fn spawn_game_timer(mut commands: Commands) {
    commands.spawn(GameTime {
        time: Timer::new(Duration::from_secs(GAME_TIME), TimerMode::Once),
    });
}

pub enum EndState {
    Player1Won,
    Player2Won,
    Draw,
}

#[derive(Event)]
pub struct GameEndEvent {
    pub end_state: EndState,
}

fn check_game_end(
    q_game_time: Query<&GameTime>,
    q_points: Query<&Points>,
    mut event_writer: EventWriter<GameEndEvent>,
) {
    let timer = q_game_time.single().unwrap();
    if timer.just_finished() {
        let points = q_points.single().unwrap();
        let end_state = {
            if points.player_1 > points.player_2 {
                EndState::Player1Won
            } else if points.player_2 > points.player_1 {
                EndState::Player2Won
            } else {
                EndState::Draw
            }
        };

        event_writer.write(GameEndEvent { end_state });
    }
}

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<GameEndEvent>()
            .add_systems(Startup, spawn_game_timer)
            .add_systems(
                Update,
                (
                    reset_p1_after_goal,
                    reset_p2_after_goal,
                    reset_ball_after_goal,
                    update_game_timer,
                    check_game_end,
                    reset_game,
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
