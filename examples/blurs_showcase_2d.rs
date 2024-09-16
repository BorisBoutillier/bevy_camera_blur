use bevy::prelude::*;

mod helpers;

fn main() {
    helpers::showcase::common_showcase_app()
        .add_systems(
            Update,
            helpers::setup_2d_scene.run_if(in_state(helpers::BlurType::Setup)),
        )
        .run();
}
