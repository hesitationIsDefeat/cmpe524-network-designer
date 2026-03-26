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

impl Point3D {
    /// Calculates the 3D Euclidean distance between this point and another.
    pub fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
