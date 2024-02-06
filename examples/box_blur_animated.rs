use std::time::Duration;

use bevy::prelude::*;
use bevy_camera_blur::*;
use bevy_tweening::*;
use helpers::{setup_blur_settings_ui, BlurSettingsUiText, ResComp};

mod helpers;

#[derive(Resource)]
struct AnimationDurationMs(u64);

fn main() {
    helpers::common_app()
        .add_plugins((BoxBlurPlugin, TweeningPlugin))
        .add_state::<GameState>()
        .insert_resource(ResComp::<BoxBlurSettings>::default())
        .insert_resource(AnimationDurationMs(500))
        .add_systems(
            Startup,
            (
                helpers::setup_3d_scene,
                setup_camera,
                setup_ui,
                setup_blur_settings_ui,
            ),
        )
        .add_systems(
            Update,
            (
                component_animator_system::<BoxBlurSettings>,
                gamestate_interaction,
                blur_settings_color,
                helpers::update_box_blur_settings, //.run_if(in_state(GameState::Menu)),
            ),
        )
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
        BoxBlurSettings::NO_BLUR,
    ));
}

fn gamestate_interaction(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
    mut duration: ResMut<AnimationDurationMs>,
    mut text: Query<&mut Text, With<AnimatedUiText>>,
) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    *text = "Animated example:\n".to_string();
    text.push_str(&format!("(Space) Toggle state: {:?}\n", state.get()));
    text.push_str(&format!(
        "(Up/Down) Animation duration: {} ms\n",
        duration.0
    ));

    if input.just_pressed(KeyCode::Space) {
        next_state.set(match state.get() {
            GameState::Menu => GameState::Game,
            GameState::Game => GameState::Menu,
        })
    }
    if input.just_pressed(KeyCode::Up) {
        duration.0 += 100;
    }
    if input.just_pressed(KeyCode::Down) {
        duration.0 = duration.0.max(100) - 100;
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
struct AnimatedUiText;

fn setup_ui(mut commands: Commands) {
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
            right: Val::Px(10.0),
            ..default()
        }),
        AnimatedUiText,
    ));
}
fn spawn_menu(mut commands: Commands, mut settings_ui: Query<&mut BlurSettingsUiText>) {
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
    settings_ui.single_mut().allow_user_interaction = true;
}
fn despawn_menu(
    mut commands: Commands,
    nodes: Query<Entity, With<Menu>>,
    mut text: Query<(&mut Text, &mut BlurSettingsUiText)>,
) {
    for entity in nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }
    let (mut text, mut settings_ui) = text.single_mut();
    text.sections[0].value = String::new();
    settings_ui.allow_user_interaction = false;
}

fn deblur(
    mut commands: Commands,
    camera: Query<(Entity, &BoxBlurSettings), With<Camera>>,
    mut stored_settings: ResMut<ResComp<BoxBlurSettings>>,
    duration: Res<AnimationDurationMs>,
) {
    let (camera_entity, settings) = camera.single();
    stored_settings.0 = *settings;
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(duration.0),
        BoxBlurLens::new(*settings, BoxBlurSettings::NO_BLUR),
    );
    commands.entity(camera_entity).insert(Animator::new(tween));
}
fn blur_settings_color(
    state: Res<State<GameState>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
    animation: Query<&Animator<BoxBlurSettings>, With<Camera>>,
) {
    let mut text = text.single_mut();
    text.sections[0].style.color = if state.get() == &GameState::Game
        || animation
            .get_single()
            .map(|animator| animator.tweenable().progress() < 1.0)
            .unwrap_or(false)
    {
        Color::GRAY
    } else {
        Color::WHITE
    }
}
fn blur(
    mut commands: Commands,
    camera: Query<Entity, With<Camera>>,
    stored_settings: Res<ResComp<BoxBlurSettings>>,
    duration: Res<AnimationDurationMs>,
) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(duration.0),
        BoxBlurLens::new(BoxBlurSettings::NO_BLUR, stored_settings.0),
    );
    let camera_entity = camera.single();
    commands.entity(camera_entity).insert(Animator::new(tween));
}
