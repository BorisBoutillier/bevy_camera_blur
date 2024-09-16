use bevy::prelude::*;
mod helpers;

fn main() {
    helpers::animation::common_animation_app()
        .add_systems(
            Update,
            helpers::setup_3d_scene.run_if(in_state(helpers::BlurType::Setup)),
        )
        .add_systems(Update, helpers::update_camera_projection)
        .run();
}
