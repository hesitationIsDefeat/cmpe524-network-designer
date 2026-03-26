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
