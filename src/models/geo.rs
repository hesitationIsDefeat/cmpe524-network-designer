//! This file contains utility code required to handle geographic functionality.

/// Represents a point or a vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    /// X-axis value
    pub x: f64,
    /// Y-axis value
    pub y: f64,
    /// Z-axis value
    pub z: f64,
}
