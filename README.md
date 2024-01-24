# Camera blur post-processing effect for Bevy

[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Jondolf/bevy_xpbd#license)

This crate provides Bevy plugins to add fullscreen post-processing blurring effects to a 2D or 3D camera.

## Algorithms

Here are the currently supporting algorithm and their associated plugin:

| algorithm | Plugin |
|-----------| ------ |
| Gaussian Blur | `GaussianBlurPlugin` |
| Box Blur | `BoxBlurPlugin` |

## Examples

See the `examples/` in the [github repository](https://github.com/borisboutillier/bevy_camera_blur).

## Bevy version compatibility

|bevy|bevy\_camera\_blur|
|----|---|
|0.12|0.1|


## License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.