//! Displays a single [`Sprite`], created from an image.

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, sprite::MaterialMesh2dBundle,
    window::PresentMode,
};
use bevy_camera_blur::*;
mod common;

fn main() {
    common::common_app()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(GaussianBlurPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, common::setup_gaussian_blur_settings_ui)
        .add_systems(Startup, common::setup_fps_ui)
        .add_systems(Update, common::update_gaussian_blur_settings)
        .add_systems(Update, common::update_fps_ui)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        GaussianBlurSettings {
            sigma: 10.,
            ..default()
        },
    ));
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}
