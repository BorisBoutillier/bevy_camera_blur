use bevy::prelude::*;

mod helpers;

fn main() {
    helpers::common_showcase_app()
        .add_systems(Startup, (setup_camera, helpers::setup_3d_scene))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}
