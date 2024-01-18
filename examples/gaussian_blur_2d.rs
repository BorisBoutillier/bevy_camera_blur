//! Displays a single [`Sprite`], created from an image.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_camera_blur::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GaussianBlurPlugin)
        .add_systems(Startup, setup)
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
            kernel_size: 40,
            sample_rate_factor: 1.,
            _webgl2_padding: 0.,
        },
    ));
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}
