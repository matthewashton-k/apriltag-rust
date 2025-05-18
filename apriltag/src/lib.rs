//! The Rusty AprilTag detector.
//!
//! The crate is built on top of [apriltag-sys](apriltag_sys).
//! It provides high level type wrappers on images, detections, and so on.
//!
//! Third-party type conversions are supported by extension crates,
//! including
//!
//! - **apriltag-nalgebra**: Add conversions from/to two dimensional byte matrix in nalgebra crate.
//! - **apriltag-image**: Add conversions from/to image types in image crate.
//!
//! # Features
//!
//! - **image**: Enables the `apriltag-image` integration for the [image](https://crates.io/crates/image) crate.
//! - **nalgebra**: Enables the `apriltag-nalgebra` integration for the [nalgebra](https://crates.io/crates/nalgebra) crate.
//! - **full**: Enables all features.

pub mod detection;
pub mod detector;
pub mod error;
pub mod families;
pub mod image_buf;
pub mod matd;
pub mod pose;
pub mod zarray;

pub use detection::Detection;
pub use detector::{Detector, DetectorBuilder};
pub use error::Error;
pub use families::Family;
pub use image_buf::Image;
pub use matd::MatdRef;
pub use pose::{Pose, PoseEstimation, TagParams};
pub use zarray::ZArray;

// Re-export apriltag-sys for users who need lower-level access
pub use apriltag_sys as sys;

// Re-export apriltag-image when the 'image' feature is enabled
#[cfg(feature = "image")]
pub use apriltag_image as image;

// Re-export apriltag-nalgebra when the 'nalgebra' feature is enabled
#[cfg(feature = "nalgebra")]
pub use apriltag_nalgebra as nalgebra;
