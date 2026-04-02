use crate::models::geo::{Bounds3D, Point3D};
use rand::seq::SliceRandom;

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

impl PlacementStrategy for KMeansPlacement {
    fn generate_positions(
        &self,
        uav_count: usize,
        users: &[Point3D],
        _bounds: &Bounds3D, // Unused in pure K-Means, but useful for other strategies
    ) -> Vec<Point3D> {
        if uav_count == 0 {
            return vec![];
        }

        // Edge Case: More UAVs than users. Just place one UAV directly on each user.
        if uav_count >= users.len() {
            let mut positions = users.to_vec();
            positions.truncate(uav_count);
            // Ensure they are at the target altitude
            for p in &mut positions {
                p.z = self.target_altitude;
            }
            return positions;
        }

        let mut rng = rand::thread_rng();

        // 1. Initialization: Pick `uav_count` random users to be the starting centroids
        let mut centroids: Vec<Point3D> = users
            .choose_multiple(&mut rng, uav_count)
            .cloned()
            .collect();

        let mut assignments = vec![0; users.len()];
        let mut changed = true;
        let mut iterations = 0;

        // 2. Optimization Loop
        while changed && iterations < self.max_iterations {
            changed = false;
            iterations += 1;

            // Step A: Assign each user to the closest centroid (UAV)
            for (user_idx, user) in users.iter().enumerate() {
                let mut min_dist = f64::MAX;
                let mut best_centroid = 0;

                for (centroid_idx, centroid) in centroids.iter().enumerate() {
                    let dist = user.distance_to(centroid);
                    if dist < min_dist {
                        min_dist = dist;
                        best_centroid = centroid_idx;
                    }
                }

                if assignments[user_idx] != best_centroid {
                    assignments[user_idx] = best_centroid;
                    changed = true;
                }
            }

            // Step B: Move centroids to the center (average) of their assigned users
            let mut new_centroids = vec![
                Point3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                };
                uav_count
            ];
            let mut counts = vec![0; uav_count];

            for (user_idx, &cluster_idx) in assignments.iter().enumerate() {
                new_centroids[cluster_idx].x += users[user_idx].x;
                new_centroids[cluster_idx].y += users[user_idx].y;
                counts[cluster_idx] += 1;
            }

            for i in 0..uav_count {
                if counts[i] > 0 {
                    centroids[i].x = new_centroids[i].x / counts[i] as f64;
                    centroids[i].y = new_centroids[i].y / counts[i] as f64;
                    // Force the Z axis to the target altitude
                    centroids[i].z = self.target_altitude;
                } else {
                    // Handle "dead" centroid (no users assigned).
                    // Teleport it to a random user to keep it useful.
                    if let Some(random_user) = users.choose(&mut rng) {
                        centroids[i] = *random_user;
                    }
                }
            }
        }

        centroids
    }
}
