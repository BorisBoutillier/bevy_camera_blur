/// functions used by multiple examples
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PresentMode,
};
use bevy_camera_blur::*;

pub fn common_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            present_mode: PresentMode::AutoNoVsync,
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(FrameTimeDiagnosticsPlugin)
    .add_systems(Startup, setup_fps_ui)
    .add_systems(Update, update_fps_ui);
    app
}
#[derive(Component)]
pub struct FpsText;

pub fn setup_fps_ui(mut commands: Commands) {
    // UI
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
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        FpsText,
    ));
}

pub fn update_fps_ui(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(sma) = fps.smoothed() {
            text.sections[0].value = format!("FPS: {sma:3.1}");
        }
    };
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
pub struct BlurUiText;

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
        BlurUiText,
    ));
}
pub fn update_gaussian_blur_settings(
    mut camera: Query<(Entity, Option<&mut GaussianBlurSettings>), With<Camera>>,
    mut text: Query<&mut Text, With<BlurUiText>>,
    mut commands: Commands,
    keycode: Res<Input<KeyCode>>,
) {
    let settings = camera.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    match settings {
        (entity, Some(mut settings)) => {
            *text = "GaussianBlurSettings (Toggle: Space)\n".to_string();
            text.push_str(&format!("(Q/A) Sigma: {}\n", settings.sigma));
            text.push_str(&format!("(W/S) Kernel size: {:?}\n", settings.kernel_size));

            if keycode.just_pressed(KeyCode::A) {
                settings.sigma -= 1.;
            }
            if keycode.just_pressed(KeyCode::Q) {
                settings.sigma += 1.;
            }
            settings.sigma = settings.sigma.clamp(0.0, 100.0);

            if keycode.just_pressed(KeyCode::S) {
                settings.kernel_size = match settings.kernel_size {
                    KernelSize::Auto => KernelSize::Auto,
                    KernelSize::Val(1) => KernelSize::Auto,
                    KernelSize::Val(v) => KernelSize::Val((v - 2).clamp(1, 401)),
                };
            }
            if keycode.just_pressed(KeyCode::W) {
                settings.kernel_size = match settings.kernel_size {
                    KernelSize::Auto => KernelSize::Val(1),
                    KernelSize::Val(v) => KernelSize::Val((v + 2).clamp(1, 401)),
                };
            }
            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).remove::<GaussianBlurSettings>();
            }
        }

        (entity, None) => {
            *text = "GaussianBlurSettings: Off (Toggle: Space)".to_string();

            if keycode.just_pressed(KeyCode::Space) {
                commands
                    .entity(entity)
                    .insert(GaussianBlurSettings::default());
            }
        }
    }
}

pub fn update_box_blur_settings(
    mut camera: Query<(Entity, Option<&mut BoxBlurSettings>), With<Camera>>,
    mut text: Query<&mut Text, With<BlurUiText>>,
    mut commands: Commands,
    keycode: Res<Input<KeyCode>>,
) {
    let settings = camera.single_mut();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    match settings {
        (entity, Some(mut settings)) => {
            *text = "BoxBlurSettings (Toggle: Space)\n".to_string();
            text.push_str(&format!("(Q/A) Kernel size: {}\n", settings.kernel_size));
            text.push_str(&format!("(W/S) N passes: {:?}\n", settings.n_passes));

            if keycode.just_pressed(KeyCode::A) {
                settings.kernel_size = settings.kernel_size.saturating_sub(2);
            }
            if keycode.just_pressed(KeyCode::Q) {
                settings.kernel_size += 2;
            }
            settings.kernel_size = settings.kernel_size.clamp(1, 401);

            if keycode.just_pressed(KeyCode::S) {
                settings.n_passes -= 1;
            }
            if keycode.just_pressed(KeyCode::W) {
                settings.n_passes += 1;
            }
            settings.n_passes = settings.n_passes.clamp(1, 5);

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).remove::<BoxBlurSettings>();
            }
        }

        (entity, None) => {
            *text = "BoxBlurSettings: Off (Toggle: Space)".to_string();

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).insert(BoxBlurSettings::default());
            }
        }
    }
}
