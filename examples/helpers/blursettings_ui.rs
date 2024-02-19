use super::*;

#[derive(Component)]
pub struct BlurSettingsUiText {
    pub allow_user_interaction: bool,
}

pub fn setup_blur_settings_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            }),
            TextSection::from_style(TextStyle {
                font_size: 14.0,
                color: Color::WHITE,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        BlurSettingsUiText {
            allow_user_interaction: true,
        },
    ));
}

fn settings_info(settings: &impl BlurSetting) -> String {
    format!(
        "\nSetting cost:\nSamplings per pixel: ~{:.1}\nPost-processing passes: {}",
        settings.sampling_per_pixel(),
        settings.passes(),
    )
}
pub fn update_gaussian_blur_settings(
    mut settings: Query<&mut GaussianBlurSettings, With<Camera>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
    keycode: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut settings) = settings.get_single_mut() {
        let mut text = text.single_mut();
        text.sections[1].value = settings_info(&*settings);
        let text = &mut text.sections[0].value;

        *text = "Gaussian Blur settings:\n".to_string();
        text.push_str(&format!("(Q/A) Kernel size: {}\n", settings.kernel_size));
        text.push_str(&format!(
            "(W/S) Sampling distance factor: {:.1}\n",
            settings.sampling_distance_factor
        ));

        if keycode.just_pressed(KeyCode::KeyA) {
            settings.kernel_size = settings.kernel_size.saturating_sub(2).clamp(1, 401);
        }
        if keycode.just_pressed(KeyCode::KeyQ) {
            settings.kernel_size = (settings.kernel_size + 2).clamp(1, 401);
        }
        if keycode.pressed(KeyCode::KeyW) {
            settings.sampling_distance_factor =
                (settings.sampling_distance_factor + time.delta_seconds() * 1.).clamp(1., 100.);
        }
        if keycode.pressed(KeyCode::KeyS) {
            settings.sampling_distance_factor =
                (settings.sampling_distance_factor - time.delta_seconds() * 1.).clamp(1., 100.);
        }
    }
}

pub fn update_box_blur_settings(
    mut settings: Query<&mut BoxBlurSettings, With<Camera>>,
    mut text: Query<(&mut Text, &BlurSettingsUiText)>,
    keycode: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut settings) = settings.get_single_mut() {
        let (mut text, settings_ui) = text.single_mut();
        text.sections[1].value = settings_info(&*settings);
        let text = &mut text.sections[0].value;

        *text = "Box Blur settings:\n".to_string();
        text.push_str(&format!("(Q/A) Kernel size: {}\n", settings.kernel_size));
        text.push_str(&format!("(W/S) passes: {:?}\n", settings.passes));

        if settings_ui.allow_user_interaction {
            if keycode.just_pressed(KeyCode::KeyA) {
                settings.kernel_size = settings.kernel_size.saturating_sub(2);
            }
            if keycode.just_pressed(KeyCode::KeyQ) {
                settings.kernel_size += 2;
            }
            settings.kernel_size = settings.kernel_size.clamp(1, 401);

            if keycode.just_pressed(KeyCode::KeyS) {
                settings.passes -= 1;
            }
            if keycode.just_pressed(KeyCode::KeyW) {
                settings.passes += 1;
            }
            settings.passes = settings.passes.clamp(1, 5);
        }
    }
}

pub fn update_kawase_blur_settings(
    mut selected: Local<usize>,
    mut settings: Query<&mut KawaseBlurSettings, With<Camera>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
    keycode: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut settings) = settings.get_single_mut() {
        let mut text = text.single_mut();
        text.sections[1].value = settings_info(&*settings);
        let text = &mut text.sections[0].value;

        *text = "Kawase Blur settings:\n".to_string();
        let kernel_str = format!(
            "Kernels: [{}]\n",
            settings
                .sampling_distances
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

        if keycode.just_pressed(KeyCode::KeyQ) {
            let v = settings.sampling_distances.len() as u32;
            settings.sampling_distances.push(v);
        }
        if keycode.just_pressed(KeyCode::KeyA) {
            settings.sampling_distances.pop();
            *selected = (*selected).min(settings.sampling_distances.len().max(1) - 1);
        }
        if keycode.just_pressed(KeyCode::KeyD) {
            *selected = selected.max(1) - 1;
        }
        if keycode.just_pressed(KeyCode::KeyF) {
            *selected = (*selected + 1).min(settings.sampling_distances.len() - 1);
        }
        if keycode.just_pressed(KeyCode::KeyW) {
            settings.sampling_distances[*selected] += 1;
        }
        if keycode.just_pressed(KeyCode::KeyS) {
            settings.sampling_distances[*selected] =
                settings.sampling_distances[*selected].max(1) - 1;
        }
    }
}

pub fn update_dual_blur_settings(
    mut settings: Query<&mut DualBlurSettings, With<Camera>>,
    mut text: Query<&mut Text, With<BlurSettingsUiText>>,
    keycode: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut settings) = settings.get_single_mut() {
        let mut text = text.single_mut();
        text.sections[1].value = settings_info(&*settings);
        let text = &mut text.sections[0].value;

        *text = "Dual Blur settings:\n".to_string();
        text.push_str(&format!(
            "(Q/A) Downsampling passes: {}\n",
            settings.downsampling_passes
        ));

        if keycode.just_pressed(KeyCode::KeyQ) {
            settings.downsampling_passes += 1
        }
        if keycode.just_pressed(KeyCode::KeyA) {
            settings.downsampling_passes = settings.downsampling_passes.max(1) - 1;
        }
    }
}
