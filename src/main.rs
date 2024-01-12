mod arena;
mod camera;
mod player;

use arena::ArenaPlugin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        // custom plugins
        .add_plugins((CameraPlugin, PlayerPlugin, ArenaPlugin))
        .add_systems(Startup, setup_physics)
        .run();
}

// A ball the player can kick around, lol
fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}
