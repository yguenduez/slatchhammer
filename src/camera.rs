use bevy::{
    app::{Plugin, Startup},
    ecs::{component::Component, system::Commands},
    math::Vec3,
    transform::components::Transform,
};
use bevy::prelude::Camera3d;

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 50.0, 35.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_camera);
    }
}
