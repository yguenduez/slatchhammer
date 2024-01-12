mod arena;
mod camera;
mod player;

use arena::ArenaPlugin;
use bevy::{
    math::vec3,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            //       Uncomment for physic colliders render debug
            //       RapierDebugRenderPlugin::default(),
        ))
        // custom plugins
        .add_plugins((CameraPlugin, PlayerPlugin, ArenaPlugin))
        .add_systems(Startup, (setup_physics, spawn_light))
        .run();
}

fn spawn_light(mut commands: Commands) {
    let light_poses = [
        vec3(20.0, 20.0, -20.0),
        vec3(-20.0, 20.0, -20.0),
        vec3(-20.0, 20.0, 20.0),
        vec3(20.0, 20.0, 20.0),
    ];

    light_poses.into_iter().for_each(|t| {
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 9000.0,
                range: 70.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_translation(t),
            ..default()
        });
    })
}

// A ball the player can kick around, lol
fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the bouncing ball. */
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let mut shape = shape::UVSphere::default();
    shape.radius = 0.5;
    let mesh = meshes.add(shape.into());
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(1.5))
        .insert(PbrBundle {
            mesh,
            material: debug_material,
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..Default::default()
        });
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
