use bevy::prelude::*;
mod helpers;

fn main() {
    helpers::animation::common_animation_app()
        .add_systems(
            Update,
            helpers::setup_2d_scene.run_if(in_state(helpers::animation::GameState::Setup)),
        )
        .run();
}
