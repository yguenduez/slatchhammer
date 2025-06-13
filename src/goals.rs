use std::f32::consts::FRAC_PI_2;

use crate::colors::{GREEN, ORANGE};
use bevy::prelude::{Cuboid, Mesh3d};
use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        event::{Event, EventReader, EventWriter},
        system::{Commands, Query, ResMut},
    },
    math::{vec3, Quat},
    pbr::{NotShadowCaster, StandardMaterial},
    render::mesh::Mesh,
    transform::components::Transform,
};
use bevy::pbr::MeshMaterial3d;
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, ColliderMassProperties},
    pipeline::CollisionEvent,
};

const MAP_SIZE_HALF: f32 = 15.0;
const GOAL_SIZE: f32 = 10.0;
const GOAL_THICKNESS: f32 = 1.0;
const GOAL_HEIGHT: f32 = 4.0;

fn build_goal_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(GOAL_SIZE, GOAL_HEIGHT * 2.0, GOAL_THICKNESS));
    let material_green = materials.add(StandardMaterial {
        base_color: GREEN,
        ..Default::default()
    });

    let material_orange = materials.add(StandardMaterial {
        base_color: ORANGE,
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
            material_orange,
        ),
        (
            Transform::from_translation(vec3(
                -MAP_SIZE_HALF * 2.0 + GOAL_THICKNESS,
                GOAL_HEIGHT * 0.5,
                0.0,
            ))
            .with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
            mesh.clone(),
            material_green,
        ),
    ];

    for (t, m, color) in transforms_with_mesh {
        commands.spawn((
            NotShadowCaster,
            Transform::from(t),
            Mesh3d(m.clone()),
            MeshMaterial3d(color.clone()),
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
            GoalType::First,
        ),
        (
            Transform::from_translation(vec3(
                -MAP_SIZE_HALF * 2.0 + GOAL_THICKNESS,
                GOAL_HEIGHT * 0.5,
                0.0,
            )),
            Collider::cuboid(GOAL_THICKNESS * 0.5, GOAL_HEIGHT, GOAL_SIZE * 0.5),
            GoalType::Second,
        ),
    ];

    for (t, c, goal_type) in transforms_with_collider {
        commands.spawn((
            c,
            RigidBody::Fixed,
            ColliderMassProperties::Mass(100.0),
            Transform::from(t),
            goal_type,
        ));
    }
}

#[derive(Component)]
pub enum GoalType {
    First,
    Second,
}

pub enum PlayerType {
    First,
    Second,
}

#[derive(Event)]
pub struct GoalEvent {
    pub amount: u32,
    pub player: PlayerType,
}

fn send_goal_event(goal_type: &GoalType, goal_event_writer: &mut EventWriter<GoalEvent>) {
    let player_type = match goal_type {
        GoalType::First => PlayerType::First,
        GoalType::Second => PlayerType::Second,
    };
    goal_event_writer.send(GoalEvent {
        amount: 1,
        player: player_type,
    });
}

fn check_collision_for_goals(
    mut collision_events: EventReader<CollisionEvent>,
    mut goal_event_writer: EventWriter<GoalEvent>,
    q_goal_type: Query<&GoalType>,
) {
    for ev in collision_events.read() {
        match ev {
            CollisionEvent::Started(first_collider, second_collider, _) => {
                if let Ok(goal_type) = q_goal_type.get(*second_collider) {
                    send_goal_event(goal_type, &mut goal_event_writer)
                }

                if let Ok(goal_type) = q_goal_type.get(*first_collider) {
                    send_goal_event(goal_type, &mut goal_event_writer)
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
pub struct GoalPlugin;
impl Plugin for GoalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (build_goal_meshes, build_goal_colliders))
            .add_systems(Update, check_collision_for_goals)
            .add_event::<GoalEvent>();
    }
}
