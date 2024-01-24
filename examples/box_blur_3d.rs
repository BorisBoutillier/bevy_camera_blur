use bevy::prelude::*;
use bevy_camera_blur::*;

mod common;
fn main() {
    common::common_app()
        .add_plugins(BoxBlurPlugin)
        .add_systems(
            Startup,
            (
                setup_camera,
                common::setup_3d_scene,
                common::setup_blur_settings_ui,
            ),
        )
        .add_systems(Update, common::update_box_blur_settings)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BoxBlurSettings::default(),
    ));
}
