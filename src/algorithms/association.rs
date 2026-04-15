//! Algorithms for associating ground users to UAVs based on network constraints.

use crate::models::geo::Point3D;
use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};

/// The common interface for all UAV-to-User association algorithms.
pub trait AssociationStrategy {
    /// Generates a binary matrix where `matrix[user_index][uav_index]` is `true`
    /// if the user is connected to that UAV.
    ///
    /// Returns a `Result` because constraints (like capacity) might make
    /// a valid association mathematically impossible.
    fn associate(
        &self,
        users: &[Point3D],
        uavs: &[Point3D],
        max_connections_per_uav: usize,
    ) -> Result<Vec<Vec<bool>>, String>;
}

/// An optimal association strategy utilizing Integer Linear Programming (ILP).
/// It minimizes the total distance between users and their connected UAVs
/// while strictly adhering to maximum connection capacities.
pub struct IlpAssociation;

impl AssociationStrategy for IlpAssociation {
    fn associate(
        &self,
        users: &[Point3D],
        uavs: &[Point3D],
        max_connections_per_uav: usize,
    ) -> Result<Vec<Vec<bool>>, String> {
        let num_users = users.len();
        let num_uavs = uavs.len();

        if num_users == 0 || num_uavs == 0 {
            return Ok(vec![]);
        }

        // Quick mathematical check before running the heavy solver
        if num_users > num_uavs * max_connections_per_uav {
            return Err(format!(
                "Infeasible: {} users exceed total network capacity of {}",
                num_users,
                num_uavs * max_connections_per_uav
            ));
        }

        let mut vars = variables!();

        // 1. Create Variables: x[user_idx][uav_idx]
        let mut x = Vec::with_capacity(num_users);
        for _ in 0..num_users {
            let mut user_vars = Vec::with_capacity(num_uavs);
            for _ in 0..num_uavs {
                // ILP requirement: Association is strictly binary (0 or 1)
                user_vars.push(vars.add(variable().binary()));
            }
            x.push(user_vars);
        }

        // 2. Define Objective: Minimize Distance
        let mut objective = Expression::from(0.0);
        for i in 0..num_users {
            for j in 0..num_uavs {
                let distance = users[i].distance_to(&uavs[j]);
                objective += x[i][j] * distance;
            }
        }

        let mut problem = vars.minimise(objective).using(default_solver);

        // 3. Constraint 1: Every user must connect to exactly ONE UAV
        for i in 0..num_users {
            let mut one_uav_expr = Expression::from(0.0);
            for j in 0..num_uavs {
                one_uav_expr += x[i][j];
            }
            problem = problem.with(one_uav_expr.eq(1.0));
        }

        // 4. Constraint 2: UAVs cannot exceed maximum connection limits
        for j in 0..num_uavs {
            let mut capacity_expr = Expression::from(0.0);
            for i in 0..num_users {
                capacity_expr += x[i][j];
            }
            problem = problem.with(capacity_expr.leq(max_connections_per_uav as f64));
        }

        // 5. Solve the Model
        let solution = problem
            .solve()
            .map_err(|_| "Solver failed to find a feasible ILP association.".to_string())?;

        // 6. Parse results into a clean binary matrix
        let mut binary_matrix = vec![vec![false; num_uavs]; num_users];
        for i in 0..num_users {
            for j in 0..num_uavs {
                // If the solver assigned a value > 0.5, it's a 1 (connected)
                if solution.value(x[i][j]) > 0.5 {
                    binary_matrix[i][j] = true;
                }
            }
        }

        Ok(binary_matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_association_by_distance() {
        let strategy = IlpAssociation;

        let users = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 100.0,
                y: 100.0,
                z: 0.0,
            },
        ];
        let uavs = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 50.0,
            },
            Point3D {
                x: 100.0,
                y: 100.0,
                z: 50.0,
            },
        ];

        let result = strategy.associate(&users, &uavs, 5).unwrap();

        assert!(result[0][0]);
        assert!(!result[0][1]);

        assert!(!result[1][0]);
        assert!(result[1][1]);
    }

    #[test]
    fn test_capacity_overflow_forces_suboptimal_distance() {
        let strategy = IlpAssociation;

        let users = vec![
            Point3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 3.0,
                y: 0.0,
                z: 0.0,
            },
        ];

        let uavs = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 50.0,
            },
            Point3D {
                x: 100.0,
                y: 0.0,
                z: 50.0,
            },
        ];

        let result = strategy.associate(&users, &uavs, 2).unwrap();

        let uav_0_connections = result.iter().filter(|user| user[0]).count();
        let uav_1_connections = result.iter().filter(|user| user[1]).count();

        assert_eq!(
            uav_0_connections, 2,
            "UAV 0 should be perfectly maxed out at 2 users"
        );
        assert_eq!(
            uav_1_connections, 1,
            "The 3rd user must be forced to the distant UAV 1 due to capacity limits"
        );
    }

    #[test]
    fn test_infeasible_network_capacity() {
        let strategy = IlpAssociation;

        let users = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };
            5
        ];

        let uavs = vec![
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 50.0
            };
            2
        ];

        let result = strategy.associate(&users, &uavs, 2);

        assert!(
            result.is_err(),
            "Should return an Err when users exceed total network capacity"
        );
    }

    #[test]
    fn test_empty_network_safe_handling() {
        let strategy = IlpAssociation;
        let result = strategy.associate(&vec![], &vec![], 5).unwrap();
        assert!(result.is_empty());
    }
}
