# AprilTag for Rust

This project provides Rust bindings to AprilTag tag detection
library. Please read the [apriltag](apriltag/README.md) README to
learn the usages and discover examples.

This repository includes the following crates.

- [apriltag](apriltag/README.md): High-level API for AprilTag library.
- [apriltag-sys](apriltag-sys/README.md): Low-level Rust bindings for
  AprilTag C library.
- [apriltag-image](apriltag-image/README.md): Add image conversion
  from/to [image](https://crates.io/crates/image) crate to apriltag crate.
- [apriltag-nalgebra](apriltag-nalgebra/README.md): Add type
  conversions from/to [nalgebra](https://crates.io/crates/nalgebra)
  crate to apriltag crate.

## Using as a Dependency

To use this project as a dependency, add the following to your `Cargo.toml`:

```toml
[dependencies]
apriltag = "0.5.0"
```

If you need access to the image or nalgebra integrations, enable the appropriate features:

```toml
[dependencies]
apriltag = { version = "0.5.0", features = ["image", "nalgebra"] }
```

Or enable all features with:

```toml
[dependencies]
apriltag = { version = "0.5.0", features = ["full"] }
```

With features enabled, you can access the sub-crates directly through the main crate:

```rust
// Access apriltag-sys
use apriltag::sys;

// With 'image' feature enabled
use apriltag::image;

// With 'nalgebra' feature enabled
use apriltag::nalgebra;
```

## License

It is distributed with a BSD 2-Clause license. Please read the
[LICENSE](LICENSE) file.
