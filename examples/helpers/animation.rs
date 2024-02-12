#![cfg(feature = "bevy_tweening")]
use std::time::Duration;

use super::*;
use bevy_tweening::*;

#[derive(Resource)]
struct AnimationDurationMs(u64);

pub fn common_animation_app() -> App {
    let mut app = common_app();
    app.add_plugins((
        GaussianBlurPlugin,
        BoxBlurPlugin,
        DualBlurPlugin,
        TweeningPlugin,
    ))
    .add_state::<BlurType>()
    .insert_resource(ResComp::<GaussianBlurSettings>::default())
    .insert_resource(ResComp::<BoxBlurSettings>::default())
    .insert_resource(ResComp::<DualBlurSettings>::default())
    .add_state::<GameState>()
    .insert_resource(AnimationDurationMs(500))
    .add_systems(
        Startup,
        (
            setup_animation_ui,
            setup_blurtype_ui,
            setup_blur_settings_ui,
        ),
    )
    .add_systems(
        Update,
        (
            component_animator_system::<BoxBlurSettings>,
            component_animator_system::<GaussianBlurSettings>,
            gamestate_interaction,
            blur_settings_color::<GaussianBlurSettings>.run_if(in_state(BlurType::Gaussian)),
            blur_settings_color::<BoxBlurSettings>.run_if(in_state(BlurType::Box)),
            blur_settings_color::<DualBlurSettings>.run_if(in_state(BlurType::Dual)),
            update_gaussian_blur_settings
                .run_if(in_state(BlurType::Gaussian).and_then(in_state(GameState::Menu))),
            update_box_blur_settings
                .run_if(in_state(BlurType::Box).and_then(in_state(GameState::Menu))),
            update_dual_blur_settings
                .run_if(in_state(BlurType::Dual).and_then(in_state(GameState::Menu))),
            update_blurtype,
            update_blurtype_ui,
        ),
    )
    .add_systems(
        OnEnter(GameState::Menu),
        (
            spawn_menu,
            animate_blur::<BoxBlurSettings, BoxBlurLens>.run_if(in_state(BlurType::Box)),
            animate_blur::<GaussianBlurSettings, GaussianBlurLens>
                .run_if(in_state(BlurType::Gaussian)),
        ),
    )
    .add_systems(
        OnExit(GameState::Menu),
        (
            despawn_menu,
            animate_deblur::<BoxBlurSettings, BoxBlurLens>.run_if(in_state(BlurType::Box)),
            animate_deblur::<GaussianBlurSettings, GaussianBlurLens>
                .run_if(in_state(BlurType::Gaussian)),
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
    .add_systems(OnExit(BlurType::Dual), del_blur::<DualBlurSettings>);
    app
}

#[derive(States, Hash, Default, Debug, PartialEq, Eq, Clone, Copy)]
enum GameState {
    #[default]
    Menu,
    Game,
}
const ANIMATABLE_BLURTYPES: [BlurType; 3] = [BlurType::Gaussian, BlurType::Box, BlurType::Dual];

fn update_blurtype(
    state: Res<State<BlurType>>,
    input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<BlurType>>,
) {
    if input.just_pressed(KeyCode::Left) {
        let mut new = state.prev();
        while !ANIMATABLE_BLURTYPES.contains(&new) {
            new = new.prev();
        }
        next_state.set(new);
    }
    if input.just_pressed(KeyCode::Right) {
        let mut new = state.next();
        while !ANIMATABLE_BLURTYPES.contains(&new) {
            new = new.next();
        }
        next_state.set(new);
    }
    if input.just_pressed(KeyCode::Key0) {
        next_state.set(ANIMATABLE_BLURTYPES[0]);
    }
    if input.just_pressed(KeyCode::Key1) {
        next_state.set(ANIMATABLE_BLURTYPES[1]);
    }
    if input.just_pressed(KeyCode::Key2) {
        next_state.set(ANIMATABLE_BLURTYPES[2]);
    }
}

fn gamestate_interaction(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
    mut duration: ResMut<AnimationDurationMs>,
    mut text: Query<&mut Text, With<AnimationUiText>>,
) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    *text = "Animation example:\n".to_string();
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
struct AnimationUiText;

fn setup_animation_ui(mut commands: Commands) {
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
        AnimationUiText,
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

fn blur_settings_color<C: Component>(
    state: Res<State<GameState>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
    animation: Query<&Animator<C>, (With<Camera>, With<C>)>,
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
fn animate_blur<C: Component + Clone + BlurSetting, L: BlurSettingLens<C>>(
    mut commands: Commands,
    camera: Query<Entity, With<Camera>>,
    res_settings: Res<ResComp<C>>,
    duration: Res<AnimationDurationMs>,
) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(duration.0),
        L::new(C::NO_BLUR, res_settings.0.clone()),
    );
    let camera_entity = camera.single();
    commands.entity(camera_entity).insert(Animator::new(tween));
}
fn animate_deblur<C: Component + Clone + BlurSetting, L: BlurSettingLens<C>>(
    mut commands: Commands,
    camera: Query<(Entity, &C), With<Camera>>,
    mut stored_settings: ResMut<ResComp<C>>,
    duration: Res<AnimationDurationMs>,
) {
    let (camera_entity, settings) = camera.single();
    stored_settings.0 = settings.clone();
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(duration.0),
        L::new(settings.clone(), C::NO_BLUR),
    );
    commands.entity(camera_entity).insert(Animator::new(tween));
}

fn add_blur<C: Component + Clone + BlurSetting>(
    mut commands: Commands,
    camera: Query<Entity, With<Camera>>,
    res_settings: Res<ResComp<C>>,
    game_state: Res<State<GameState>>,
) {
    commands
        .entity(camera.single())
        .insert(match game_state.get() {
            GameState::Menu => res_settings.0.clone(),
            GameState::Game => C::NO_BLUR,
        });
}
fn del_blur<C: Component + Clone>(
    mut commands: Commands,
    camera: Query<(Entity, &C), With<Camera>>,
    mut res_settings: ResMut<ResComp<C>>,
) {
    let (camera, settings) = camera.single();
    res_settings.0 = settings.clone();
    commands.entity(camera).remove::<C>();
}
