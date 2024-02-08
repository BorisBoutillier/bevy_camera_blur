#![allow(dead_code)]

/// Set of functions that are used by multiple examples
///
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    math::Vec3A,
    prelude::*,
    render::primitives::{Aabb, Sphere},
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
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()),
                ..default()
            }),
    )
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
fn parse_scene(scene_path: String) -> (String, usize) {
    if scene_path.contains('#') {
        let gltf_and_scene = scene_path.split('#').collect::<Vec<_>>();
        if let Some((last, path)) = gltf_and_scene.split_last() {
            if let Some(index) = last
                .strip_prefix("Scene")
                .and_then(|index| index.parse::<usize>().ok())
            {
                return (path.join("#"), index);
            }
        }
    }
    (scene_path, 0)
}

pub fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(scene_path) = std::env::args().nth(1) {
        println!("\nLoading scene from path: {}\n", scene_path);
        commands.spawn(SceneBundle {
            scene: asset_server.load(scene_path),
            ..default()
        });
    } else {
        println!("\nA default simple 3D scene is spawned.\n");
        println!("You can provide a gltf scene to spawn as first argument on the command line");
        println!("Similarly as for the bevy scene_viewer example\n");
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
    }
    // Directional light that will be animated
    commands.spawn((DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 40000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            0.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        ..default()
    },));
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
pub fn update_camera_projection(
    meshes: Query<(&GlobalTransform, Option<&Aabb>), With<Handle<Mesh>>>,
    mut events: EventReader<AssetEvent<Scene>>,
    mut camera: Query<(&mut Projection, &mut Transform), With<Camera3d>>,
) {
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id: _ } = event {}
        if meshes.iter().any(|(_, maybe_aabb)| maybe_aabb.is_none()) {
            return;
        }

        let mut min = Vec3A::splat(f32::MAX);
        let mut max = Vec3A::splat(f32::MIN);
        for (transform, maybe_aabb) in &meshes {
            let aabb = maybe_aabb.unwrap();
            // If the Aabb had not been rotated, applying the non-uniform scale would produce the
            // correct bounds. However, it could very well be rotated and so we first convert to
            // a Sphere, and then back to an Aabb to find the conservative min and max points.
            let sphere = Sphere {
                center: Vec3A::from(transform.transform_point(Vec3::from(aabb.center))),
                radius: transform.radius_vec3a(aabb.half_extents),
            };
            let aabb = Aabb::from(sphere);
            min = min.min(aabb.min());
            max = max.max(aabb.max());
        }

        let size = (max - min).length();
        let aabb = Aabb::from_min_max(Vec3::from(min), Vec3::from(max));

        let mut perspective_projection = PerspectiveProjection::default();
        perspective_projection.far = perspective_projection.far.max(size * 10.0);

        let (mut projection, mut transform) = camera.single_mut();
        *projection = perspective_projection.into();
        *transform =
            Transform::from_translation(Vec3::from(aabb.center) + size * Vec3::new(0.5, 0.25, 0.5))
                .looking_at(Vec3::from(aabb.center), Vec3::Y);
    }
}
