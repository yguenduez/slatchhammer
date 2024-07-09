mod arena;
mod camera;
mod colors;
mod constants;
mod game_state;
mod goals;
mod player;
mod points;
mod sprint;
mod ui;

use arena::ArenaPlugin;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{
    math::vec3,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;
use bevy_vector_shapes::ShapePlugin;
use camera::CameraPlugin;
use game_state::GameStatePlugin;
use goals::GoalPlugin;
use player::PlayerPlugin;
use points::PointsPlugin;
use sprint::StatePlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#slatchhammer-canvas".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(ShapePlugin::default())
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            //       Uncomment for physic colliders render debug
            // RapierDebugRenderPlugin::default(),
        ))
        // custom plugins
        .add_plugins((
            CameraPlugin,
            PlayerPlugin,
            ArenaPlugin,
            GoalPlugin,
            PointsPlugin,
            GameStatePlugin,
            UiPlugin,
            StatePlugin,
        ))
        .add_systems(Startup, (spawn_ball, spawn_light))
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
#[derive(Component)]
pub struct Ball;
fn spawn_ball(
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
    let mut shape = Sphere::default();
    shape.radius = 0.5;
    let mesh = meshes.add(shape);
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Ball)
        .insert(Collider::ball(0.5))
        .insert(Velocity::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
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
        RenderAssetUsages::default(),
    )
}
