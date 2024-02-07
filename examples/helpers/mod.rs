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

mod blursettings_ui;
use blursettings_ui::*;

pub mod animation;
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
    // Camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },));
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
    // Camera
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource, Default)]
pub struct ResComp<C: Component>(pub C);

const BLURSTATE_COUNT: usize = 5;
#[derive(States, Hash, Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlurType {
    None = 0,
    #[default]
    Gaussian = 1,
    Box = 2,
    Kawase = 3,
    Dual = 4,
}
impl BlurType {
    pub fn next(&self) -> Self {
        ((*self as usize + 1) % BLURSTATE_COUNT).into()
    }
    pub fn prev(&self) -> Self {
        ((*self as usize + BLURSTATE_COUNT - 1) % BLURSTATE_COUNT).into()
    }
}
impl From<usize> for BlurType {
    fn from(value: usize) -> Self {
        use BlurType::*;
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

#[derive(Component)]
pub struct BlurTypeUiText;

pub fn update_blurtype_ui(
    state: Res<State<BlurType>>,
    mut text: Query<&mut Text, With<BlurTypeUiText>>,
) {
    if state.is_changed() {
        text.single_mut().sections[0].value = format!("{:?}", state.get());
    }
}

pub fn setup_blurtype_ui(mut commands: Commands) {
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
            "(Left/Right or number) Change blur type",
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    );
}
