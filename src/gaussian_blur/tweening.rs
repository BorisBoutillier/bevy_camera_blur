#![cfg(feature = "bevy_tweening")]
use bevy_tweening::Lens;

use crate::GaussianBlurSettings;

/// A bevy_tweening Lens implementation to allow animation of the gaussian blur.
///
/// This will tweens the `sigma`, `kernel_size`, and `texel_step` attributes
/// of the GaussianBlurSettings.
/// # Example
///
/// Implement `Lens` for a custom type:
///
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_tweening::*;
///pub fn blur(
///    mut commands: Commands,
///    camera: Query<Entity, (With<Camera>, With<GaussianBlurSettings>)>,
///) {
///    let tween = Tween::new(
///        EaseFunction::QuadraticInOut,
///        Duration::from_millis(BLUR_ANIMATION_DURATION),
///        GaussianBlurLens {
///            start: NO_BLUR,
///            end: BLUR,
///        },
///    );
///    let camera_entity = camera.single();
///    commands.entity(camera_entity).insert(Animator::new(tween));
///}
/// ```
pub struct GaussianBlurLens {
    /// Gaussian blur settings at the end of the tweening
    pub start: GaussianBlurSettings,
    /// Gaussian blur settings at the end of the tweening
    pub end: GaussianBlurSettings,
}
impl Lens<GaussianBlurSettings> for GaussianBlurLens {
    fn lerp(&mut self, target: &mut GaussianBlurSettings, ratio: f32) {
        target.sigma = self.start.sigma + (self.end.sigma - self.start.sigma) * ratio;
        target.kernel_size = (self.start.kernel_size as f32
            + (self.end.kernel_size as f32 - self.start.kernel_size as f32) * ratio)
            as u32;
        target.sample_rate_factor = self.start.sample_rate_factor
            + (self.end.sample_rate_factor - self.start.sample_rate_factor) * ratio;
    }
}
