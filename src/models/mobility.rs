use crate::models::geo::{Bounds3D, Point3D};

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PositionUpdateType {
    RANDOM,
    DIRECTIONAL,
}
pub struct MobilityGenerator;

impl MobilityGenerator {
    pub fn generate_locations(
        start_time: f64,
        end_time: f64,
        update_interval: f64,
        initial_positions: &[Point3D],
        velocities: &[Point3D],
        update_types: &[PositionUpdateType],
        bounds: &Bounds3D,
    ) -> Vec<Vec<Point3D>> {
        let steps = ((end_time - start_time) / update_interval).floor() as usize;
        let mut all_user_paths = Vec::with_capacity(initial_positions.len());

        for i in 0..initial_positions.len() {
            let mut user_path = Vec::with_capacity(steps + 1);
            let mut current_pos = initial_positions[i];
            let velocity = velocities[i];
            let update_type = update_types[i];

            user_path.push(current_pos);

            for _ in 0..steps {
                current_pos = match update_type {
                    PositionUpdateType::RANDOM => {
                        Self::calculate_random_step(&current_pos, &velocity, update_interval)
                    }
                    PositionUpdateType::DIRECTIONAL => {
                        Self::calculate_directional_step(&current_pos, &velocity, update_interval)
                    }
                };

                current_pos = Self::clamp_to_bounds(&current_pos, bounds);
                user_path.push(current_pos);
            }

            all_user_paths.push(user_path);
        }

        all_user_paths
    }

    fn calculate_random_step(current: &Point3D, velocity: &Point3D, dt: f64) -> Point3D {
        let mut rng = rand::thread_rng();

        let dx = rng.gen_range(-1.0..=1.0) * velocity.x * dt;
        let dy = rng.gen_range(-1.0..=1.0) * velocity.y * dt;
        let dz = rng.gen_range(-1.0..=1.0) * velocity.z * dt;

        Point3D {
            x: current.x + dx,
            y: current.y + dy,
            z: current.z + dz,
        }
    }

    fn calculate_directional_step(current: &Point3D, velocity: &Point3D, dt: f64) -> Point3D {
        let mut rng = rand::thread_rng();

        let mut new_pos = *current;
        let move_on_x = rng.gen_bool(0.5);

        if move_on_x {
            new_pos.x += velocity.x * dt;
        } else {
            new_pos.y += velocity.y * dt;
        }

        new_pos
    }

    fn clamp_to_bounds(point: &Point3D, bounds: &Bounds3D) -> Point3D {
        Point3D {
            x: point.x.clamp(bounds.min_x, bounds.max_x),
            y: point.y.clamp(bounds.min_y, bounds.max_y),
            z: point.z.clamp(bounds.min_z, bounds.max_z),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::geo::{Bounds3D, Point3D};

    // Helper to generate a standard test boundary
    fn mock_bounds() -> Bounds3D {
        Bounds3D {
            min_x: 0.0,
            max_x: 100.0,
            min_y: 0.0,
            max_y: 100.0,
            min_z: 0.0,
            max_z: 50.0,
        }
    }

    #[test]
    fn test_clamp_to_bounds() {
        let bounds = mock_bounds();

        // Point entirely inside bounds should not change
        let valid_point = Point3D {
            x: 50.0,
            y: 50.0,
            z: 25.0,
        };
        assert_eq!(
            MobilityGenerator::clamp_to_bounds(&valid_point, &bounds),
            valid_point
        );

        // Point out of bounds should be forced to the exact boundary edge
        let out_of_bounds = Point3D {
            x: -10.0,
            y: 150.0,
            z: 60.0,
        };
        let clamped = MobilityGenerator::clamp_to_bounds(&out_of_bounds, &bounds);

        assert_eq!(clamped.x, 0.0); // Clamped to min_x
        assert_eq!(clamped.y, 100.0); // Clamped to max_y
        assert_eq!(clamped.z, 50.0); // Clamped to max_z
    }

    #[test]
    fn test_generate_locations_step_count() {
        let bounds = mock_bounds();
        let initial = vec![Point3D {
            x: 50.0,
            y: 50.0,
            z: 25.0,
        }];
        let velocities = vec![Point3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }];
        let types = vec![PositionUpdateType::RANDOM];

        // 10 seconds total, 2 second interval = 5 generated steps.
        // Total path length should be 6 (Initial + 5 steps).
        let paths = MobilityGenerator::generate_locations(
            0.0,
            10.0,
            2.0,
            &initial,
            &velocities,
            &types,
            &bounds,
        );

        assert_eq!(paths.len(), 1, "Should generate paths for exactly 1 user");
        assert_eq!(
            paths[0].len(),
            6,
            "Path length should be (duration / interval) + 1"
        );
    }

    #[test]
    fn test_zero_velocity_means_no_movement() {
        let bounds = mock_bounds();
        let start_pos = Point3D {
            x: 50.0,
            y: 50.0,
            z: 25.0,
        };

        let initial = vec![start_pos];
        // Velocity is strictly zero
        let velocities = vec![Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }];

        // Test both update types
        let paths_random = MobilityGenerator::generate_locations(
            0.0,
            10.0,
            1.0,
            &initial,
            &velocities,
            &vec![PositionUpdateType::RANDOM],
            &bounds,
        );
        let paths_directional = MobilityGenerator::generate_locations(
            0.0,
            10.0,
            1.0,
            &initial,
            &velocities,
            &vec![PositionUpdateType::DIRECTIONAL],
            &bounds,
        );

        // Every point in the path should be exactly the initial position
        assert!(paths_random[0].iter().all(|&p| p == start_pos));
        assert!(paths_directional[0].iter().all(|&p| p == start_pos));
    }

    #[test]
    fn test_directional_step_preserves_z_axis() {
        let initial = Point3D {
            x: 50.0,
            y: 50.0,
            z: 25.0,
        };
        let velocity = Point3D {
            x: 10.0,
            y: 10.0,
            z: 10.0,
        }; // Note: Z velocity exists

        let next_step = MobilityGenerator::calculate_directional_step(&initial, &velocity, 1.0);

        // According to your logic, DIRECTIONAL only moves X or Y, never Z.
        assert_eq!(
            next_step.z, 25.0,
            "Z axis should remain unchanged during DIRECTIONAL updates"
        );

        // We also know it must have moved exactly velocity * dt on either X or Y, but not both.
        let x_changed = next_step.x != initial.x;
        let y_changed = next_step.y != initial.y;

        assert!(
            x_changed ^ y_changed,
            "It should move on exactly one axis (XOR)"
        );
    }

    #[test]
    fn test_bounds_are_never_exceeded() {
        let bounds = mock_bounds();
        let initial = vec![Point3D {
            x: 50.0,
            y: 50.0,
            z: 25.0,
        }];
        // Extreme velocity to ensure it tries to break the bounds on every tick
        let velocities = vec![Point3D {
            x: 9999.0,
            y: 9999.0,
            z: 9999.0,
        }];
        let types = vec![PositionUpdateType::RANDOM];

        let paths = MobilityGenerator::generate_locations(
            0.0,
            100.0,
            1.0,
            &initial,
            &velocities,
            &types,
            &bounds,
        );

        for point in &paths[0] {
            assert!(point.x >= bounds.min_x && point.x <= bounds.max_x);
            assert!(point.y >= bounds.min_y && point.y <= bounds.max_y);
            assert!(point.z >= bounds.min_z && point.z <= bounds.max_z);
        }
    }
}
