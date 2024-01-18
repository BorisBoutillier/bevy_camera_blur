//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::time::Duration;

use bevy::prelude::*;
use bevy_camera_blur::*;
use bevy_tweening::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TweeningPlugin)
        .add_plugins(GaussianBlurPlugin)
        .add_systems(Startup, setup)
        .add_state::<GameState>()
        .add_systems(Update, component_animator_system::<GaussianBlurSettings>)
        .add_systems(Update, game_interaction.run_if(in_state(GameState::Game)))
        .add_systems(OnEnter(GameState::Menu), (spawn_menu, blur))
        .add_systems(Update, menu_interaction.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), (despawn_menu, deblur))
        .run();
}

#[derive(States, Hash, Default, Debug, PartialEq, Eq, Clone, Copy)]
enum GameState {
    #[default]
    Menu,
    Game,
}
/// set up a simple 3D scene
fn setup(
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
    // Spawn a 3D camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        GaussianBlurSettings::NO_BLUR,
    ));
}

fn game_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if input.any_just_pressed([KeyCode::Escape, KeyCode::Space, KeyCode::Return])
        || mouse_input.just_pressed(MouseButton::Left)
    {
        next_state.set(GameState::Menu);
    }
}

#[derive(Component)]
struct Menu;

fn menu_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
    interactions: Query<&Interaction, Changed<Interaction>>,
) {
    if input.any_just_pressed([KeyCode::Escape, KeyCode::Space, KeyCode::Return]) {
        next_state.set(GameState::Game);
    }
    for interaction in interactions.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Game);
        }
    }
}
fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: Color::MIDNIGHT_BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Continue",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::ANTIQUE_WHITE,
                            ..default()
                        },
                    ));
                });
        });
}
fn despawn_menu(mut commands: Commands, nodes: Query<Entity, With<Menu>>) {
    for entity in nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

const BLUR: GaussianBlurSettings = GaussianBlurSettings {
    sigma: 10.0,
    kernel_size: 40,
    sample_rate_factor: 1.0,
    _webgl2_padding: 0.,
};
fn deblur(mut commands: Commands, camera: Query<Entity, With<Camera>>) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(500),
        GaussianBlurLens {
            start: BLUR,
            end: GaussianBlurSettings::NO_BLUR,
        },
    );
    let camera_entity = camera.single();
    commands.entity(camera_entity).insert(Animator::new(tween));
}
fn blur(mut commands: Commands, camera: Query<Entity, With<Camera>>) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(500),
        GaussianBlurLens {
            start: GaussianBlurSettings::NO_BLUR,
            end: BLUR,
        },
    );
    let camera_entity = camera.single();
    commands.entity(camera_entity).insert(Animator::new(tween));
}
