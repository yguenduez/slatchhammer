use bevy::math::{vec3, Vec3};

pub const PLAYER1_STARTING_POINT: Vec3 = vec3(-10.0, 1.0, 0.);
pub const PLAYER2_STARTING_POINT: Vec3 = vec3(10.0, 1.0, 0.);
pub const BALL_STARTING_POINT: Vec3 = vec3(0.0, 4.0, 0.0);
pub const BALL_STARTING_VELOCITY: Vec3 = vec3(0.0, 10.0, 0.0);
pub const GAME_TIME: u64 = 120;
pub const DISPLAY_DESPAWN_TIME: f32 = 5.0;
