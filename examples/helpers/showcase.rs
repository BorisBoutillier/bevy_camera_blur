use super::*;

pub fn common_showcase_app() -> App {
    let mut app = common_app();
    app.add_plugins((
        GaussianBlurPlugin,
        BoxBlurPlugin,
        KawaseBlurPlugin,
        DualBlurPlugin,
    ))
    .init_state::<BlurType>()
    .insert_resource(ResComp::<GaussianBlurSettings>::default())
    .insert_resource(ResComp::<BoxBlurSettings>::default())
    .insert_resource(ResComp::<KawaseBlurSettings>::default())
    .insert_resource(ResComp::<DualBlurSettings>::default())
    .add_systems(Startup, (setup_blurtype_ui, setup_blur_settings_ui))
    .add_systems(
        Update,
        (
            update_gaussian_blur_settings.run_if(in_state(BlurType::Gaussian)),
            update_kawase_blur_settings.run_if(in_state(BlurType::Kawase)),
            update_box_blur_settings.run_if(in_state(BlurType::Box)),
            update_dual_blur_settings.run_if(in_state(BlurType::Dual)),
            update_blurtype,
            update_blurtype_ui,
        ),
    )
    .add_systems(
        OnEnter(BlurType::Gaussian),
        add_blur::<GaussianBlurSettings>,
    )
    .add_systems(OnExit(BlurType::Gaussian), del_blur::<GaussianBlurSettings>)
    .add_systems(OnEnter(BlurType::Box), add_blur::<BoxBlurSettings>)
    .add_systems(OnExit(BlurType::Box), del_blur::<BoxBlurSettings>)
    .add_systems(OnEnter(BlurType::Dual), add_blur::<DualBlurSettings>)
    .add_systems(OnExit(BlurType::Dual), del_blur::<DualBlurSettings>)
    .add_systems(OnEnter(BlurType::Kawase), add_blur::<KawaseBlurSettings>)
    .add_systems(OnExit(BlurType::Kawase), del_blur::<KawaseBlurSettings>);
    app
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}

fn add_blur<C: Component + Clone>(
    mut commands: Commands,
    camera: Query<Entity, With<Camera>>,
    res_settings: Res<ResComp<C>>,
) {
    commands
        .entity(camera.single())
        .insert(res_settings.0.clone());
}
fn del_blur<C: Component + Clone>(
    mut commands: Commands,
    camera: Query<(Entity, &C), With<Camera>>,
    mut res_settings: ResMut<ResComp<C>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
) {
    let (camera, settings) = camera.single();
    res_settings.0 = settings.clone();
    commands.entity(camera).remove::<C>();
    text.single_mut().sections[0].value = "".to_string();
}

fn update_blurtype(
    state: Res<State<BlurType>>,
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<BlurType>>,
) {
    if input.just_pressed(KeyCode::ArrowLeft) {
        next_state.set(state.prev());
    }
    if input.just_pressed(KeyCode::ArrowRight) {
        next_state.set(state.next());
    }
    if input.just_pressed(KeyCode::Digit0) {
        next_state.set(0.into());
    }
    if input.just_pressed(KeyCode::Digit1) {
        next_state.set(1.into());
    }
    if input.just_pressed(KeyCode::Digit2) {
        next_state.set(2.into());
    }
    if input.just_pressed(KeyCode::Digit3) {
        next_state.set(3.into());
    }
    if input.just_pressed(KeyCode::Digit4) {
        next_state.set(4.into());
    }
}
