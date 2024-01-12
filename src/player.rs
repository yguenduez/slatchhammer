use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        query::With,
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
    transform::{components::Transform, TransformBundle},
};

use bevy_rapier3d::{
    dynamics::{ExternalForce, GravityScale, LockedAxes, RigidBody, Velocity},
    geometry::Collider,
};

use crate::camera::MainCamera;

#[derive(Component)]
struct Player;

#[derive(Component, Default)]
struct PlayerInput {
    movement: Vec3,
}

fn movement_input(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut PlayerInput, With<Player>>,
    cameras: Query<&Transform, With<MainCamera>>,
) {
    let camera_transform = cameras.single();
    let forward = camera_transform.right();
    let rotation = Quat::from_axis_angle(Vec3::Y, forward.y);

    for mut player_input in query.iter_mut() {
        let x = movement_axis(&input, KeyCode::A, KeyCode::D);
        let z = movement_axis(&input, KeyCode::W, KeyCode::S);
        let dir = vec3(x, 0.0, z).normalize_or_zero();
        let dir = rotation * dir;
        player_input.movement = dir;
    }
}

fn apply_movement(
    mut query: Query<(&PlayerInput, &mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    const PLAYER_MOVEMENT_SPEED: f32 = 10.;
    for (input, mut transform, mut velocity) in query.iter_mut() {
        let norm_input = input.movement.normalize_or_zero();

        let desired_velocity = norm_input * PLAYER_MOVEMENT_SPEED;
        velocity.linvel = Vec3::lerp(
            velocity.linvel,
            desired_velocity,
            time.delta_seconds() * 10.0,
        );

        transform.rotation = Quat::from_rotation_y(f32::atan2(norm_input.x, norm_input.z));
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
    let material = materials.add(StandardMaterial {
        base_color: Color::BLUE,
        ..Default::default()
    });
    commands
        .spawn((
            Player,
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
            mesh,
            material,
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
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
