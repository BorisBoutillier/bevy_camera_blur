#![allow(dead_code)]
/// Set of functions that are used by multiple examples
///
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PresentMode,
};
use bevy_camera_blur::*;
pub mod showcase;

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
    settings: Query<&KawaseBlurSettings, With<Camera>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
    _keycode: Res<Input<KeyCode>>,
) {
    let settings = settings.single();
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    *text = "Kawase Blur settings:\n".to_string();
    text.push_str(&format!("Kernels: {:?}\n", settings.kernels));
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
