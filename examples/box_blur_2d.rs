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
                common::setup_2d_scene,
                common::setup_blur_settings_ui,
            ),
        )
        .add_systems(Update, common::update_box_blur_settings)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), BoxBlurSettings::default()));
}