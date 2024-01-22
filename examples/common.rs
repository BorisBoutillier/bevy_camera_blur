/// functions used by multiple examples
use bevy::prelude::*;
use bevy_camera_blur::*;

#[derive(Component)]
pub struct GaussianBlurUiText;

pub fn setup_gaussian_blur_settings_ui(mut commands: Commands) {
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
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        GaussianBlurUiText,
    ));
}
pub fn update_gaussian_blur_settings(
    mut camera: Query<(Entity, Option<&mut GaussianBlurSettings>), With<Camera>>,
    mut text: Query<&mut Text, With<GaussianBlurUiText>>,
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

            if keycode.pressed(KeyCode::A) {
                settings.sigma -= 1.;
            }
            if keycode.pressed(KeyCode::Q) {
                settings.sigma += 1.;
            }
            settings.sigma = settings.sigma.clamp(0.0, 100.0);

            if keycode.pressed(KeyCode::S) {
                settings.kernel_size = match settings.kernel_size {
                    KernelSize::Auto => KernelSize::Auto,
                    KernelSize::Val(1) => KernelSize::Auto,
                    KernelSize::Val(v) => KernelSize::Val((v - 1).clamp(1, 401)),
                };
            }
            if keycode.pressed(KeyCode::W) {
                settings.kernel_size = match settings.kernel_size {
                    KernelSize::Auto => KernelSize::Val(1),
                    KernelSize::Val(v) => KernelSize::Val((v + 1).clamp(1, 401)),
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
