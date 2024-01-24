use std::time::Duration;

use bevy::prelude::*;
use bevy_camera_blur::*;
use bevy_tweening::*;

mod common;

fn main() {
    common::common_app()
        .add_plugins(TweeningPlugin)
        .add_state::<GameState>()
        .add_systems(Startup, (common::setup_3d_scene, setup_camera, setup_ui))
        .add_systems(Update, component_animator_system::<GaussianBlurSettings>)
        .add_systems(Update, gamestate_interaction)
        .add_systems(OnEnter(GameState::Menu), (spawn_menu, blur))
        .add_systems(OnExit(GameState::Menu), (despawn_menu, deblur))
        .run();
}

#[derive(States, Hash, Default, Debug, PartialEq, Eq, Clone, Copy)]
enum GameState {
    #[default]
    Menu,
    Game,
}
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        GaussianBlurSettings::NO_BLUR,
    ));
}

fn gamestate_interaction(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(match state.get() {
            GameState::Menu => GameState::Game,
            GameState::Game => GameState::Menu,
        })
    }
}

#[derive(Component)]
struct Menu;

fn setup_ui(mut commands: Commands) {
    commands.spawn((TextBundle::from_section(
        "(Space) Toggle GameState",
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
    }),));
}
fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
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
                        "Menu",
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
    kernel_size: KernelSize::Auto,
};
fn deblur(mut commands: Commands, camera: Query<Entity, With<Camera>>) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(500),
        GaussianBlurLens::new(BLUR, GaussianBlurSettings::NO_BLUR),
    );
    let camera_entity = camera.single();
    commands.entity(camera_entity).insert(Animator::new(tween));
}
fn blur(mut commands: Commands, camera: Query<Entity, With<Camera>>) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(500),
        GaussianBlurLens::new(GaussianBlurSettings::NO_BLUR, BLUR),
    );
    let camera_entity = camera.single();
    commands.entity(camera_entity).insert(Animator::new(tween));
}
