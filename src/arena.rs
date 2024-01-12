use std::f32::consts::FRAC_PI_2;

use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::{vec2, vec3, Quat},
    pbr::{MaterialMeshBundle, NotShadowCaster, StandardMaterial},
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
    transform::components::Transform,
};

const MAP_SIZE_HALF: f32 = 15.0;

fn build_arena_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wall_height = 4.0;

    let mesh = meshes.add(shape::Quad::new(vec2(MAP_SIZE_HALF * 2.0, wall_height)).into());
    let material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..Default::default()
    });

    // wall right
    commands.spawn((
        NotShadowCaster,
        MaterialMeshBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(vec3(MAP_SIZE_HALF, wall_height * 0.5, 0.0))
                .with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
            material: material.clone(),
            ..Default::default()
        },
    ));
    // wall right
    commands.spawn((
        NotShadowCaster,
        MaterialMeshBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(vec3(-MAP_SIZE_HALF, wall_height * 0.5, 0.0))
                .with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
            material: material.clone(),
            ..Default::default()
        },
    ));
    // wall up
    commands.spawn((
        NotShadowCaster,
        MaterialMeshBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(vec3(0.0, wall_height * 0.5, -MAP_SIZE_HALF)),
            material: material.clone(),
            ..Default::default()
        },
    ));
    // wall bottom
    commands.spawn((
        NotShadowCaster,
        MaterialMeshBundle {
            mesh: mesh.clone(),
            transform: Transform::from_translation(vec3(0.0, wall_height * 0.5, MAP_SIZE_HALF)),
            material: material.clone(),
            ..Default::default()
        },
    ));
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (build_arena_walls));
    }
}
