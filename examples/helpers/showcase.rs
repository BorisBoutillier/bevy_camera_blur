use super::*;

const BLURSTATE_COUNT: usize = 5;
#[derive(States, Hash, Default, Debug, PartialEq, Eq, Clone, Copy)]
enum BlurState {
    None = 0,
    #[default]
    Gaussian = 1,
    Box = 2,
    Kawase = 3,
    Dual = 4,
}
impl BlurState {
    fn next(&self) -> Self {
        ((*self as usize + 1) % BLURSTATE_COUNT).into()
    }
    fn prev(&self) -> Self {
        ((*self as usize + BLURSTATE_COUNT - 1) % BLURSTATE_COUNT).into()
    }
}
impl From<usize> for BlurState {
    fn from(value: usize) -> Self {
        use BlurState::*;
        match value {
            0 => None,
            1 => Gaussian,
            2 => Box,
            3 => Kawase,
            4 => Dual,
            _ => panic!("No state for value {}", value),
        }
    }
}

#[derive(Resource, Default)]
struct ResComp<C: Component>(C);

pub fn common_showcase_app() -> App {
    let mut app = common_app();
    app.add_plugins(GaussianBlurPlugin)
        .add_plugins(BoxBlurPlugin)
        .add_plugins(KawaseBlurPlugin)
        .add_plugins(DualBlurPlugin)
        .add_state::<BlurState>()
        .insert_resource(ResComp::<GaussianBlurSettings>::default())
        .insert_resource(ResComp::<BoxBlurSettings>::default())
        .insert_resource(ResComp::<KawaseBlurSettings>::default())
        .insert_resource(ResComp::<DualBlurSettings>::default())
        .add_systems(Startup, (setup_showcase_ui, setup_blur_settings_ui))
        .add_systems(
            Update,
            (
                update_gaussian_blur_settings.run_if(in_state(BlurState::Gaussian)),
                update_kawase_blur_settings.run_if(in_state(BlurState::Kawase)),
                update_box_blur_settings.run_if(in_state(BlurState::Box)),
                update_dual_blur_settings.run_if(in_state(BlurState::Dual)),
                update_blur_state,
                update_blurtype_ui,
            ),
        )
        .add_systems(
            OnEnter(BlurState::Gaussian),
            add_blur::<GaussianBlurSettings>,
        )
        .add_systems(
            OnExit(BlurState::Gaussian),
            del_blur::<GaussianBlurSettings>,
        )
        .add_systems(OnEnter(BlurState::Box), add_blur::<BoxBlurSettings>)
        .add_systems(OnExit(BlurState::Box), del_blur::<BoxBlurSettings>)
        .add_systems(OnEnter(BlurState::Dual), add_blur::<DualBlurSettings>)
        .add_systems(OnExit(BlurState::Dual), del_blur::<DualBlurSettings>)
        .add_systems(OnEnter(BlurState::Kawase), add_blur::<KawaseBlurSettings>)
        .add_systems(OnExit(BlurState::Kawase), del_blur::<KawaseBlurSettings>);
    app
}

#[derive(Component)]
struct BlurTypeUiText;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
}

fn setup_showcase_ui(mut commands: Commands) {
    // Top Middle text showing current Blur Type
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                top: Val::Px(20.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::ANTIQUE_WHITE,

                        ..default()
                    },
                ),
                BlurTypeUiText,
            ));
        });
    // Bottom Right text showing commands to change Blur Type
    commands.spawn(
        TextBundle::from_section(
            "(Left/Right/0/1/2/3/4) Change blur type",
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
    );
}
fn update_blurtype_ui(
    state: Res<State<BlurState>>,
    mut text: Query<&mut Text, With<BlurTypeUiText>>,
) {
    if state.is_changed() {
        text.single_mut().sections[0].value = format!("{:?}", state.get());
    }
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

fn update_blur_state(
    state: Res<State<BlurState>>,
    input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<BlurState>>,
) {
    if input.just_pressed(KeyCode::Left) {
        next_state.set(state.prev());
    }
    if input.just_pressed(KeyCode::Right) {
        next_state.set(state.next());
    }
    if input.just_pressed(KeyCode::Key0) {
        next_state.set(0.into());
    }
    if input.just_pressed(KeyCode::Key1) {
        next_state.set(1.into());
    }
    if input.just_pressed(KeyCode::Key2) {
        next_state.set(2.into());
    }
    if input.just_pressed(KeyCode::Key3) {
        next_state.set(3.into());
    }
    if input.just_pressed(KeyCode::Key4) {
        next_state.set(4.into());
    }
}

pub fn setup_2d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}

pub fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a simple 3D scene
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

#[derive(Component)]
pub struct BlurSettingsUiText;

pub fn setup_blur_settings_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        BlurSettingsUiText,
    ));
}
pub fn update_gaussian_blur_settings(
    mut settings: Query<&mut GaussianBlurSettings, With<Camera>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
    keycode: Res<Input<KeyCode>>,
) {
    let mut settings = settings.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    *text = "Gaussian Blur settings:\n".to_string();
    text.push_str(&format!("(Q/A) Kernel size: {}\n", settings.kernel_size));

    if keycode.just_pressed(KeyCode::A) {
        settings.kernel_size = settings.kernel_size.saturating_sub(2).clamp(1, 401);
    }
    if keycode.just_pressed(KeyCode::Q) {
        settings.kernel_size = (settings.kernel_size + 2).clamp(1, 401);
    }
}

pub fn update_box_blur_settings(
    mut settings: Query<&mut BoxBlurSettings, With<Camera>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
    keycode: Res<Input<KeyCode>>,
) {
    let mut settings = settings.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    *text = "Box Blur settings:\n".to_string();
    text.push_str(&format!("(Q/A) Kernel size: {}\n", settings.kernel_size));
    text.push_str(&format!("(W/S) passes: {:?}\n", settings.passes));

    if keycode.just_pressed(KeyCode::A) {
        settings.kernel_size = settings.kernel_size.saturating_sub(2);
    }
    if keycode.just_pressed(KeyCode::Q) {
        settings.kernel_size += 2;
    }
    settings.kernel_size = settings.kernel_size.clamp(1, 401);

    if keycode.just_pressed(KeyCode::S) {
        settings.passes -= 1;
    }
    if keycode.just_pressed(KeyCode::W) {
        settings.passes += 1;
    }
    settings.passes = settings.passes.clamp(1, 5);
}

pub fn update_kawase_blur_settings(
    mut selected: Local<usize>,
    mut settings: Query<&mut KawaseBlurSettings, With<Camera>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
    keycode: Res<Input<KeyCode>>,
) {
    let mut settings = settings.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    *text = "Kawase Blur settings:\n".to_string();
    let kernel_str = format!(
        "Kernels: [{}]\n",
        settings
            .kernels
            .iter()
            .enumerate()
            .map(|(i, v)| {
                if i == *selected {
                    format!("({})", v)
                } else {
                    v.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    );
    text.push_str(&kernel_str);
    text.push_str("(Q/A) Change kernels length\n");
    text.push_str("(W/S) Change selected kernel value\n");
    text.push_str("(D/F) Change selected kernel entry\n");
    if keycode.just_pressed(KeyCode::Q) {
        let v = settings.kernels.len() as u32;
        settings.kernels.push(v);
    }
    if keycode.just_pressed(KeyCode::A) {
        settings.kernels.pop();
        *selected = (*selected).min(settings.kernels.len().max(1) - 1);
    }
    if keycode.just_pressed(KeyCode::D) {
        *selected = selected.max(1) - 1;
    }
    if keycode.just_pressed(KeyCode::F) {
        *selected = (*selected + 1).min(settings.kernels.len() - 1);
    }
    if keycode.just_pressed(KeyCode::W) {
        settings.kernels[*selected] += 1;
    }
    if keycode.just_pressed(KeyCode::S) {
        settings.kernels[*selected] = settings.kernels[*selected].max(1) - 1;
    }
}

pub fn update_dual_blur_settings(
    mut settings: Query<&mut DualBlurSettings, With<Camera>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
    keycode: Res<Input<KeyCode>>,
) {
    let mut settings = settings.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    *text = "Dual Blur settings:\n".to_string();
    text.push_str(&format!(
        "(Q/A) Downsampling passes: {}\n",
        settings.downsampling_passes
    ));

    if keycode.just_pressed(KeyCode::Q) {
        settings.downsampling_passes += 1
    }
    if keycode.just_pressed(KeyCode::A) {
        settings.downsampling_passes = settings.downsampling_passes.max(1) - 1;
    }
}
