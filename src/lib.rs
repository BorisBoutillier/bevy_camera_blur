#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
//! This crate provides Bevy plugins to add fullscreen post-processing blurring effects
//! to a 2D or 3D camera.
//!
//! # Algorithms
//!
//! Here are the currently supporting algorithm and their associated plugin:
//!
//! | algorithm | Plugin |
//! |-----------| ------ |
//! | Gaussian Blur | [`GaussianBlurPlugin`] |
//! | Box Blur | [`BoxBlurPlugin`] |
//! | Kawase Blur | [`KawaseBlurPlugin`] |
//! | Dual Blur | [`DualBlurPlugin`] |
//!
//! # Features flags
//!
//! * **`bevy_tweening`** -
//!   When enabled `Lens` implementations are provided for each effect to use with the `bevy_tweening` crate.
//!   This adds a dependency on the `bevy_tweening` crate.
//!
//!
mod gaussian_blur;
pub use gaussian_blur::*;
mod box_blur;
pub use box_blur::*;
mod kawase_blur;
pub use kawase_blur::*;
mod dual_blur;
pub use dual_blur::*;

/// Provides a const settings
pub trait BlurSetting {
    /// Setting values that will not create any blur
    /// and that will not run the associated
    /// post-processing pipeline
    const NO_BLUR: Self;
    /// Provides for this setting an estimation of
    /// the mean number of texture sampling per pixel,
    /// Taking into account all the passes.
    ///
    /// This should only be used during dev to check
    /// the quality/cost compromise.
    fn sampling_per_pixel(&self) -> f32;
    /// Provides for this setting the number of
    /// post-processing passes that are needed.
    ///
    /// This should only be used during dev to check
    /// the quality/cost compromise.
    fn passes(&self) -> u32;
}

/// Can create a Lens from a `start` and an `end` setting
#[cfg(feature = "bevy_tweening")]
pub trait BlurSettingLens<C>: bevy_tweening::Lens<C> + Send + Sync + 'static {
    /// Create a new Lens from a `start` setting and an `end` setting
    fn new(start: C, end: C) -> Self;
}
