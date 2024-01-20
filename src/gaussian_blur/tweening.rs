#![cfg(feature = "bevy_tweening")]
use bevy_tweening::Lens;

use crate::{GaussianBlurSettings, KernelSize};

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
    start: GaussianBlurSettings,
    /// Gaussian blur settings at the end of the tweening
    end: GaussianBlurSettings,
}
impl GaussianBlurLens {
    /// Creates a new Lens to tween between the provided 'start' and 'end' setting.
    pub fn new(start: GaussianBlurSettings, end: GaussianBlurSettings) -> Self {
        GaussianBlurLens {
            start: start.make_concrete(),
            end: end.make_concrete(),
        }
    }
}
impl Lens<GaussianBlurSettings> for GaussianBlurLens {
    fn lerp(&mut self, target: &mut GaussianBlurSettings, ratio: f32) {
        target.sigma = self.start.sigma + (self.end.sigma - self.start.sigma) * ratio;
        target.kernel_size = match (self.start.kernel_size, self.end.kernel_size) {
            (KernelSize::Val(v1), KernelSize::Val(v2)) => {
                let v = (v1 as f32 + (v2 as f32 - v1 as f32) * ratio) as u32;
                if v % 2 == 0 {
                    KernelSize::Val(v + 1)
                } else {
                    KernelSize::Val(v)
                }
            }
            _ => panic!("kernel_size are expected to be both Val(x)"),
        };
    }
}
