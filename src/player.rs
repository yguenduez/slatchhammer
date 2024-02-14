use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, Input},
    math::{vec3, Quat, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
    time::Time,
    transform::components::Transform,
};

use bevy_rapier3d::{
    dynamics::{ExternalForce, GravityScale, LockedAxes, RigidBody, Velocity},
    geometry::Collider,
};

use crate::{
    camera::MainCamera,
    constants::{PLAYER1_STARTING_POINT, PLAYER2_STARTING_POINT, PLAYER_MOVEMENT_SPEED},
};

#[derive(Component)]
pub struct Player1;

#[derive(Component)]
pub struct Player2;

#[derive(Component, Default)]
struct PlayerInput {
    movement: Vec3,
    current_velocity: f32,
}

fn movement_input(
    input: Res<Input<KeyCode>>,
    mut query_p1: Query<&mut PlayerInput, (With<Player1>, Without<Player2>)>,
    mut query_p2: Query<&mut PlayerInput, (With<Player2>, Without<Player1>)>,
    camera: Query<&Transform, With<MainCamera>>,
) {
    let camera_transform = camera.single();
    let forward = camera_transform.right();
    let rotation = Quat::from_axis_angle(Vec3::Y, forward.y);

    for mut player_input in query_p1.iter_mut() {
        let (x, z, vel) = dir_and_speed(
            &input,
            KeyCode::A,
            KeyCode::D,
            KeyCode::W,
            KeyCode::S,
            KeyCode::ShiftLeft,
        );
        let dir = vec3(x, 0.0, z).normalize_or_zero();
        let dir = rotation * dir;
        player_input.movement = dir;
        player_input.current_velocity = vel;
    }
    for mut player_input in query_p2.iter_mut() {
        let (x, z, vel) = dir_and_speed(
            &input,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::ShiftRight,
        );
        let dir = vec3(x, 0.0, z).normalize_or_zero();
        let dir = rotation * dir;
        player_input.movement = dir;
        player_input.current_velocity = vel;
    }
}

fn apply_movement(
    mut query: Query<(&PlayerInput, &mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    for (input, mut transform, mut velocity) in query.iter_mut() {
        let norm_input = input.movement.normalize_or_zero();

        let desired_velocity = norm_input * input.current_velocity;
        velocity.linvel = Vec3::lerp(
            velocity.linvel,
            desired_velocity,
            time.delta_seconds() * 10.0,
        );

        transform.rotation = Quat::from_rotation_y(f32::atan2(norm_input.x, norm_input.z));
    }
}

fn dir_and_speed(
    input: &Input<KeyCode>,
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
    sprint: KeyCode,
) -> (f32, f32, f32) {
    (
        movement_axis(input, left, right),
        movement_axis(input, up, down),
        adapt_velocity(input, sprint),
    )
}

fn adapt_velocity(input: &Input<KeyCode>, sprint_button: KeyCode) -> f32 {
    if input.pressed(sprint_button) {
        PLAYER_MOVEMENT_SPEED * 2.0
    } else {
        PLAYER_MOVEMENT_SPEED
    }
}

fn movement_axis(input: &Input<KeyCode>, left: KeyCode, right: KeyCode) -> f32 {
    match (input.pressed(left), input.pressed(right)) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        _ => 0f32,
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Cylinder::default().into());
    let material_green = materials.add(StandardMaterial {
        base_color: Color::GREEN,
        ..Default::default()
    });
    let material_orange = materials.add(StandardMaterial {
        base_color: Color::ORANGE,
        ..Default::default()
    });

    commands
        .spawn((
            Player1,
            PlayerInput::default(),
            RigidBody::Dynamic,
            Collider::capsule(Vec3::ZERO, Vec3::Y, 0.5),
            Velocity::default(),
            ExternalForce {
                force: Vec3::ZERO,
                torque: Vec3::ZERO,
            },
            GravityScale(1.0),
            LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Z
                | LockedAxes::ROTATION_LOCKED_Y,
        ))
        .insert(PbrBundle {
            mesh: mesh.clone(),
            material: material_green,
            transform: Transform::from_translation(PLAYER1_STARTING_POINT),
            ..Default::default()
        });
    commands
        .spawn((
            Player2,
            PlayerInput::default(),
            RigidBody::Dynamic,
            Collider::capsule(Vec3::ZERO, Vec3::Y, 0.5),
            Velocity::default(),
            ExternalForce {
                force: Vec3::ZERO,
                torque: Vec3::ZERO,
            },
            GravityScale(1.0),
            LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Z
                | LockedAxes::ROTATION_LOCKED_Y,
        ))
        .insert(PbrBundle {
            mesh: mesh.clone(),
            material: material_orange,
            transform: Transform::from_translation(PLAYER2_STARTING_POINT),
            ..Default::default()
        });
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (apply_movement, movement_input));
    }
}
