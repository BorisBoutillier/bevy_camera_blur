use bevy::prelude::*;

mod helpers;

fn main() {
    helpers::showcase::common_showcase_app()
        .add_systems(
            Update,
            helpers::setup_3d_scene.run_if(in_state(helpers::animation::GameState::Setup)),
        )
        .add_systems(Update, helpers::update_camera_projection)
        .run();
}
