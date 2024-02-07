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
pub trait NoBlurSetting {
    /// Setting configuration that will not run the associated
    /// post-processing pipeline
    const NO_BLUR: Self;
}

/// Can create a Lens from a `start` and an `end` setting
#[cfg(feature = "bevy_tweening")]
pub trait BlurSettingLens<C>: bevy_tweening::Lens<C> + Send + Sync + 'static {
    /// Create a new Lens from a `start` setting and an `end` setting
    fn new(start: C, end: C) -> Self;
}
