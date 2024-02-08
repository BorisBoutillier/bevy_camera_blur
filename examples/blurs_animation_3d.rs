use bevy::prelude::*;
mod helpers;

fn main() {
    helpers::animation::common_animation_app()
        .add_systems(Startup, helpers::setup_3d_scene)
        .add_systems(Update, helpers::update_camera_projection)
        .run();
}
