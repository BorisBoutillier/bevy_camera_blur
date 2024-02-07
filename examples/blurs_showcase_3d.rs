use bevy::prelude::*;

mod helpers;

fn main() {
    helpers::showcase::common_showcase_app()
        .add_systems(Startup, helpers::setup_3d_scene)
        .run();
}
