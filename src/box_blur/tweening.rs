#![cfg(feature = "bevy_tweening")]
use bevy_tweening::Lens;

use crate::BoxBlurSettings;

/// A `bevy_tweening` Lens implementation to allow animation of the box blur.
///
/// This will tweens the `sigma`, `kernel_size`, and `texel_step` attributes
/// of the [`BoxBlurSettings`].
/// # Example
///
/// System that will create an animation from a no blur to a  10-sigma blur effect
///
/// ```
///# use bevy::prelude::*;
///# use bevy_tweening::*;
///# use bevy_camera_blur::*;
///# use std::time::Duration;
///
///pub fn blur(
///    mut commands: Commands,
///    camera: Query<Entity, (With<Camera>, With<BoxBlurSettings>)>,
///) {
///    let tween = Tween::new(
///        EaseFunction::QuadraticInOut,
///        Duration::from_millis(500),
///        BoxBlurLens::new(
///             BoxBlurSettings::NO_BLUR,
///             BoxBlurSettings::default(),
///        )
///    );
///    let camera_entity = camera.single();
///    commands.entity(camera_entity).insert(Animator::new(tween));
///}
/// ```
pub struct BoxBlurLens {
    /// Box blur settings at the end of the tweening
    start: BoxBlurSettings,
    /// Box blur settings at the end of the tweening
    end: BoxBlurSettings,
}
impl crate::BlurSettingLens<BoxBlurSettings> for BoxBlurLens {
    fn new(start: BoxBlurSettings, end: BoxBlurSettings) -> Self {
        BoxBlurLens {
            start: start.create_concrete(),
            end: end.create_concrete(),
        }
    }
}
impl Lens<BoxBlurSettings> for BoxBlurLens {
    fn lerp(&mut self, target: &mut BoxBlurSettings, ratio: f32) {
        target.passes = (self.start.passes as f32
            + (self.end.passes as f32 - self.start.passes as f32) * ratio)
            .round() as u32;
        target.kernel_size = {
            let v1 = self.start.kernel_size;
            let v2 = self.end.kernel_size;
            let v = (v1 as f32 + (v2 as f32 - v1 as f32) * ratio).round() as u32;
            if v % 2 == 0 {
                v + 1
            } else {
                v
            }
        };
    }
}
