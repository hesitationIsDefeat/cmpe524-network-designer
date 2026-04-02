use crate::models::geo::{Bounds3D, Point3D};

/// The common interface for all UAV placement algorithms.
pub trait PlacementStrategy {
    /// Calculates the optimal starting positions for a set of UAVs.
    fn generate_positions(
        &self,
        uav_count: usize,
        users: &[Point3D],
        bounds: &Bounds3D,
    ) -> Vec<Point3D>;
}

/// A placement strategy that uses K-Means clustering to group users
/// and places a UAV at the geographic center of each cluster.
pub struct KMeansPlacement {
    /// The maximum number of times the algorithm will adjust positions before giving up.
    pub max_iterations: usize,
    /// The default altitude (Z-axis) to set for the UAVs once X and Y are found.
    pub target_altitude: f64,
}
