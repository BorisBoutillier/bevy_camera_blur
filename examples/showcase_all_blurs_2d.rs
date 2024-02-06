use bevy::prelude::*;

mod helpers;

fn main() {
    helpers::common_showcase_app()
        .add_systems(Startup, (setup_camera, helpers::setup_2d_scene))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
