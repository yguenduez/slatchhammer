use std::f32::consts::FRAC_PI_2;

use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    math::{vec2, vec3, Quat},
    pbr::{MaterialMeshBundle, NotShadowCaster, PbrBundle, StandardMaterial},
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
    transform::components::Transform,
};
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, ColliderMassProperties},
};

#[derive(Component)]
struct Goal;

const MAP_SIZE_HALF: f32 = 15.0;
const GOAL_SIZE: f32 = 10.0;
const GOAL_THICKNESS: f32 = 1.0;
const GOAL_HEIGHT: f32 = 4.0;

fn build_goal_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Box::new(GOAL_SIZE, GOAL_HEIGHT * 2.0, GOAL_THICKNESS).into());
    let material = materials.add(StandardMaterial {
        base_color: Color::BLUE,
        ..Default::default()
    });

    let transforms_with_mesh = [
        (
            Transform::from_translation(vec3(
                MAP_SIZE_HALF * 2.0 - GOAL_THICKNESS,
                GOAL_HEIGHT * 0.5,
                0.0,
            ))
            .with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
            mesh.clone(),
        ),
        (
            Transform::from_translation(vec3(
                -MAP_SIZE_HALF * 2.0 + GOAL_THICKNESS,
                GOAL_HEIGHT * 0.5,
                0.0,
            ))
            .with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
            mesh.clone(),
        ),
    ];

    for (t, m) in transforms_with_mesh {
        commands.spawn((
            NotShadowCaster,
            MaterialMeshBundle {
                mesh: m,
                transform: t,
                material: material.clone(),
                ..Default::default()
            },
        ));
    }
}
fn build_goal_colliders(mut commands: Commands) {
    let transforms_with_collider = [
        (
            Transform::from_translation(vec3(
                MAP_SIZE_HALF * 2.0 - GOAL_THICKNESS,
                GOAL_HEIGHT * 0.5,
                0.0,
            )),
            Collider::cuboid(GOAL_THICKNESS * 0.5, GOAL_HEIGHT, GOAL_SIZE * 0.5),
        ),
        (
            Transform::from_translation(vec3(
                -MAP_SIZE_HALF * 2.0 + GOAL_THICKNESS,
                GOAL_HEIGHT * 0.5,
                0.0,
            )),
            Collider::cuboid(GOAL_THICKNESS * 0.5, GOAL_HEIGHT, GOAL_SIZE * 0.5),
        ),
    ];

    for (t, c) in transforms_with_collider {
        commands.spawn((
            c,
            RigidBody::Fixed,
            ColliderMassProperties::Mass(100.0),
            PbrBundle {
                transform: t,
                ..Default::default()
            },
        ));
    }
}

pub struct GoalPlugin;
impl Plugin for GoalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (build_goal_meshes, build_goal_colliders));
    }
}
