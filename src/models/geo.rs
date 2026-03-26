//! This file contains utility code required to handle geographic functionality.

/// Represents a point or a directional vector in 3D space.
///
/// # Examples
///
/// ```
/// # use cmpe524_network_designer::models::geo::Point3D;
/// let uav_position = Point3D { x: 10.0, y: 50.0, z: 100.0 };
/// assert_eq!(uav_position.z, 100.0);
/// ```
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
    ///
    /// This is primarily used to determine if two UAVs are within wireless
    /// communication range of each other.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmpe524_network_designer::models::geo::Point3D;
    /// let uav_1 = Point3D { x: 0.0, y: 0.0, z: 0.0 };
    /// let uav_2 = Point3D { x: 3.0, y: 4.0, z: 0.0 };
    ///
    /// // The distance should form a standard 3-4-5 right triangle
    /// let distance = uav_1.distance_to(&uav_2);
    /// assert_eq!(distance, 5.0);
    /// ```
    pub fn distance_to(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Represents the physical boundaries of the 3D simulation area.
///
/// This is used to ensure that generated mobility paths do not cause
/// UAVs or users to wander outside the defined network zone.
///
/// # Examples
///
/// ```
/// # use cmpe524_network_designer::models::geo::Bounds3D;
/// let city_bounds = Bounds3D {
///     min_x: -1000.0, max_x: 1000.0,
///     min_y: -1000.0, max_y: 1000.0,
///     min_z: 0.0,     max_z: 500.0, // Altitude limit of 500 units
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Bounds3D {
    /// X-axis lower bound
    pub min_x: f64,
    /// X-axis upper bound
    pub max_x: f64,
    /// Y-axis lower bound
    pub min_y: f64,
    // Y-axis upper bound
    pub max_y: f64,
    /// Z-axis lower bound
    pub min_z: f64,
    /// Z-axis upper bound
    pub max_z: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_self_is_zero() {
        let p = Point3D {
            x: 10.5,
            y: -5.0,
            z: 3.14,
        };
        assert_eq!(p.distance_to(&p), 0.0);
    }

    #[test]
    fn test_distance_single_axis() {
        let p1 = Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let p2 = Point3D {
            x: 5.0,
            y: 0.0,
            z: 0.0,
        };

        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_distance_3d_space() {
        let p1 = Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let p2 = Point3D {
            x: 1.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(p1.distance_to(&p2), 3.0);
    }

    #[test]
    fn test_distance_with_negative_coordinates() {
        let p1 = Point3D {
            x: -2.0,
            y: -3.0,
            z: -4.0,
        };
        let p2 = Point3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let distance = p1.distance_to(&p2);

        let expected = 50.0_f64.sqrt();
        assert!((distance - expected).abs() < f64::EPSILON);

        assert_eq!(distance, p2.distance_to(&p1));
    }

    #[test]
    fn test_bounds_creation() {
        let bounds = Bounds3D {
            min_x: -100.0,
            max_x: 100.0,
            min_y: -50.0,
            max_y: 50.0,
            min_z: 0.0,
            max_z: 200.0,
        };

        assert!(bounds.min_x < bounds.max_x);
        assert_eq!(bounds.max_z, 200.0);
    }
}
