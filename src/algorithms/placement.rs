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
