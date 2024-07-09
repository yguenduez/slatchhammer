use bevy::input::ButtonInput;
use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        query::{With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    input::keyboard::KeyCode,
    math::{vec3, Quat, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    prelude::*,
    render::mesh::Mesh,
    time::Time,
    transform::components::Transform,
};
use bevy_rapier3d::{
    dynamics::{ExternalForce, GravityScale, LockedAxes, RigidBody, Velocity},
    geometry::Collider,
};

use crate::colors::{GREEN, ORANGE};
use crate::{
    camera::MainCamera,
    constants::{PLAYER1_STARTING_POINT, PLAYER2_STARTING_POINT, PLAYER_MOVEMENT_SPEED},
    sprint::{ApplySprintEvent, ShowBars, SprintState},
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
    input: Res<ButtonInput<KeyCode>>,
    mut query_p1: Query<
        (Entity, &mut PlayerInput, &SprintState),
        (With<Player1>, Without<Player2>),
    >,
    mut query_p2: Query<
        (Entity, &mut PlayerInput, &SprintState),
        (With<Player2>, Without<Player1>),
    >,
    time: Res<Time>,
    camera: Query<&Transform, With<MainCamera>>,
    mut event_writer: EventWriter<ApplySprintEvent>,
) {
    let camera_transform = camera.single();
    let forward = camera_transform.right();
    let rotation = Quat::from_axis_angle(Vec3::Y, forward.y);
    let frame_time = time.delta_seconds();

    for (entity, mut player_input, stamina) in query_p1.iter_mut() {
        let (x, z) = wanted_player_direction(
            &input,
            KeyCode::KeyA,
            KeyCode::KeyD,
            KeyCode::KeyW,
            KeyCode::KeyS,
        );
        let mut velocity = PLAYER_MOVEMENT_SPEED;
        if player_wants_to_sprint(&input, KeyCode::ShiftLeft) {
            velocity = change_velocity(stamina, frame_time, entity, &mut event_writer);
        }
        let dir = vec3(x, 0.0, z).normalize_or_zero();
        let dir = rotation * dir;
        player_input.movement = dir;
        player_input.current_velocity = velocity;
    }
    for (entity, mut player_input, stamina) in query_p2.iter_mut() {
        let (x, z) = wanted_player_direction(
            &input,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
        );
        let mut velocity = PLAYER_MOVEMENT_SPEED;
        if player_wants_to_sprint(&input, KeyCode::ShiftRight) {
            velocity = change_velocity(stamina, frame_time, entity, &mut event_writer);
        }
        let dir = vec3(x, 0.0, z).normalize_or_zero();
        let dir = rotation * dir;
        player_input.movement = dir;
        player_input.current_velocity = velocity;
    }
}

fn player_wants_to_sprint(input: &ButtonInput<KeyCode>, space: KeyCode) -> bool {
    input.pressed(space)
}

fn change_velocity(
    stamina: &SprintState,
    stamina_change: f32,
    target_entity: Entity,
    event_writer: &mut EventWriter<ApplySprintEvent>,
) -> f32 {
    let mut vel = PLAYER_MOVEMENT_SPEED;
    if stamina.is_available() {
        vel *= 2.0;
        let ev = ApplySprintEvent {
            amount: -stamina_change,
            target: target_entity,
        };
        event_writer.send(ev);
    }
    vel
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

fn wanted_player_direction(
    input: &ButtonInput<KeyCode>,
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
) -> (f32, f32) {
    (
        movement_axis(input, left, right),
        movement_axis(input, up, down),
    )
}

fn movement_axis(input: &ButtonInput<KeyCode>, left: KeyCode, right: KeyCode) -> f32 {
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
    let mesh = meshes.add(Cylinder::default());
    let material_green = materials.add(StandardMaterial {
        base_color: GREEN,
        ..Default::default()
    });
    let material_orange = materials.add(StandardMaterial {
        base_color: ORANGE,
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
        })
        .insert(SprintState::default())
        .insert(ShowBars);
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
        })
        .insert(SprintState::default())
        .insert(ShowBars);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (apply_movement, movement_input));
    }
}
