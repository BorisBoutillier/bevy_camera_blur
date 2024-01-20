#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
//! This crate provides Bevy plugins to add post-processing blurring effects
//! to a 2D or 3D camera.
//!
//! # Algorithms
//!
//! Here are the currently supporting algorithm and their associated plugin:
//!
//! | algorithm | Plugin |
//! |-----------| ------ |
//! | Gaussian Blur | [`GaussianBlurPlugin`] |
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
