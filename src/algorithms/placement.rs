//! Algorithms for determining the initial physical placement of UAVs.
//!
//! This module utilizes the Strategy Pattern via the [`PlacementStrategy`] trait.
//! This allows the simulation to hot-swap different placement algorithms
//! (like K-Means, Grid, or Random) depending on the selected configuration.

use crate::models::geo::{Bounds3D, Point3D};
use rand::seq::SliceRandom;

/// The common interface for all UAV placement algorithms.
pub trait PlacementStrategy {
    /// Calculates the optimal starting positions for a set of UAVs.
    ///
    /// # Arguments
    ///
    /// * `uav_count` - The number of UAVs available to deploy.
    /// * `users` - A slice containing the initial physical locations of all users.
    /// * `bounds` - The physical boundaries of the simulation space.
    ///
    /// # Returns
    ///
    /// A vector of `Point3D` representing the starting coordinates for each UAV.
    fn generate_positions(
        &self,
        uav_count: usize,
        users: &[Point3D],
        bounds: &Bounds3D,
    ) -> Vec<Point3D>;
}

/// A placement strategy that uses K-Means clustering to group users
/// and places a UAV at the geographic center of each cluster.
///
/// This approach minimizes the initial distance between users and their
/// closest UAV, which generally improves signal strength and reduces latency.
///
/// # Examples
///
/// ```
/// # use cmpe524_network_designer::models::geo::{Bounds3D, Point3D};
/// # use cmpe524_network_designer::algorithms::placement::{KMeansPlacement, PlacementStrategy};
/// let users = vec![
///     Point3D { x: 0.0, y: 0.0, z: 0.0 },
///     Point3D { x: 10.0, y: 10.0, z: 0.0 },
/// ];
/// let bounds = Bounds3D { min_x: 0.0, max_x: 100.0, min_y: 0.0, max_y: 100.0, min_z: 0.0, max_z: 50.0 };
///
/// let strategy = KMeansPlacement {
///     max_iterations: 100,
///     target_altitude: 50.0,
/// };
///
/// let uav_positions = strategy.generate_positions(2, &users, &bounds);
/// assert_eq!(uav_positions.len(), 2);
/// assert_eq!(uav_positions[0].z, 50.0); // Verifies target altitude
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_bounds() -> Bounds3D {
        Bounds3D {
            min_x: 0.0,
            max_x: 100.0,
            min_y: 0.0,
            max_y: 100.0,
            min_z: 0.0,
            max_z: 100.0,
        }
    }

    #[test]
    fn test_zero_uavs() {
        let strategy = KMeansPlacement {
            max_iterations: 10,
            target_altitude: 50.0,
        };
        let users = vec![Point3D {
            x: 10.0,
            y: 10.0,
            z: 0.0,
        }];

        let positions = strategy.generate_positions(0, &users, &mock_bounds());
        assert!(
            positions.is_empty(),
            "Should return empty vector when 0 UAVs are requested"
        );
    }

    #[test]
    fn test_more_uavs_than_users() {
        let strategy = KMeansPlacement {
            max_iterations: 10,
            target_altitude: 75.0,
        };
        let users = vec![
            Point3D {
                x: 10.0,
                y: 10.0,
                z: 0.0,
            },
            Point3D {
                x: 20.0,
                y: 20.0,
                z: 0.0,
            },
        ];

        // Request 5 UAVs for only 2 users
        let positions = strategy.generate_positions(5, &users, &mock_bounds());

        assert_eq!(
            positions.len(),
            2,
            "Should only place as many UAVs as there are users"
        );
        assert_eq!(
            positions[0].z, 75.0,
            "Altitude should be forced to target_altitude"
        );
        assert_eq!(positions[1].z, 75.0);
    }

    #[test]
    fn test_kmeans_clustering_logic() {
        let strategy = KMeansPlacement {
            max_iterations: 100,
            target_altitude: 50.0,
        };

        // Group 1 is centered at (0, 0)
        // Group 2 is centered at (100, 100)
        let users = vec![
            Point3D {
                x: -2.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 98.0,
                y: 100.0,
                z: 0.0,
            },
            Point3D {
                x: 102.0,
                y: 100.0,
                z: 0.0,
            },
        ];

        let positions = strategy.generate_positions(2, &users, &mock_bounds());

        assert_eq!(positions.len(), 2);

        // K-Means should perfectly identify the centers of these two clusters
        // Because of the random initial state, we don't know if positions[0] is Group 1 or Group 2
        let has_group_1 = positions
            .iter()
            .any(|p| p.x == 0.0 && p.y == 0.0 && p.z == 50.0);
        let has_group_2 = positions
            .iter()
            .any(|p| p.x == 100.0 && p.y == 100.0 && p.z == 50.0);

        assert!(
            has_group_1,
            "Failed to find the center of cluster 1 at (0,0)"
        );
        assert!(
            has_group_2,
            "Failed to find the center of cluster 2 at (100,100)"
        );
    }
}
