use std::f32::consts::FRAC_PI_2;

use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::{vec2, vec3, Quat},
    pbr::{MaterialMeshBundle, NotShadowCaster, PbrBundle, StandardMaterial},
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
    transform::{components::Transform, TransformBundle},
};
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, ColliderMassProperties},
};

const MAP_SIZE_HALF: f32 = 15.0;

fn build_arena_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_height = 4.0;

    let mesh = meshes.add(shape::Quad::new(vec2(MAP_SIZE_HALF * 2.0, wall_height * 2.0)).into());
    let mesh_long =
        meshes.add(shape::Quad::new(vec2(MAP_SIZE_HALF * 4.0, wall_height * 2.0)).into());
    let material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });

    let transforms_with_mesh = [
        (
            Transform::from_translation(vec3(MAP_SIZE_HALF * 2.0, wall_height * 0.5, 0.0))
                .with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
            mesh.clone(),
        ),
        (
            Transform::from_translation(vec3(-MAP_SIZE_HALF * 2.0, wall_height * 0.5, 0.0))
                .with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
            mesh.clone(),
        ),
        (
            Transform::from_translation(vec3(0.0, wall_height * 0.5, -MAP_SIZE_HALF)),
            mesh_long.clone(),
        ),
        (
            Transform::from_translation(vec3(0.0, wall_height * 0.5, MAP_SIZE_HALF)),
            mesh_long.clone(),
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

fn build_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(200.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -0.1, 0.0)));

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(100.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..Default::default()
    });
}

fn build_collider_walls(mut commands: Commands) {
    let wall_thickness = 0.5;
    let wall_thickness_half = wall_thickness * 0.5;
    let transforms_with_collider = [
        (
            Transform::from_translation(vec3(MAP_SIZE_HALF * 2.0 + wall_thickness_half, 0.0, 0.0)),
            Collider::cuboid(wall_thickness, 10., MAP_SIZE_HALF),
        ),
        (
            Transform::from_translation(vec3(-MAP_SIZE_HALF * 2.0 - wall_thickness_half, 0.0, 0.0)),
            Collider::cuboid(wall_thickness, 10., MAP_SIZE_HALF),
        ),
        (
            Transform::from_translation(vec3(0.0, 0.0, MAP_SIZE_HALF + wall_thickness_half)),
            Collider::cuboid(MAP_SIZE_HALF * 2.0, 10., wall_thickness),
        ),
        (
            Transform::from_translation(vec3(0.0, 0.0, -MAP_SIZE_HALF - wall_thickness_half)),
            Collider::cuboid(MAP_SIZE_HALF * 2.0, 10., wall_thickness),
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

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Startup,
            (build_arena_walls, build_collider_walls, build_ground),
        );
    }
}
