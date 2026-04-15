pub mod config;
use cmpe524_network_designer::algorithms::association::{AssociationStrategy, IlpAssociation};
use cmpe524_network_designer::algorithms::placement::KMeansPlacement;
use cmpe524_network_designer::algorithms::placement::PlacementStrategy;
use cmpe524_network_designer::models::geo::Bounds3D;
use cmpe524_network_designer::models::geo::Point3D;
use rand::Rng;

fn main() {
    println!("Loading Configuration...");

    let system_params = config::load_system_params("config/system_params.toml");
    let constraints = config::load_constraints("config/constraints.toml");

    println!("✅ Loaded {} UAVs.", system_params.uav_amount);
    println!(
        "✅ Target Success Rate: {}%",
        constraints.min_overall_success_rate * 100.0
    );

    let user_points: Vec<Point3D>;

    let bounds: Bounds3D = system_params.area.to_bounds3d();

    if system_params.user.auto_generate {
        println!(
            "🎲 Auto-generating {} random user locations...",
            system_params.user.user_amount
        );
        let mut rng = rand::thread_rng();
        let mut generated_users = Vec::with_capacity(system_params.user.user_amount);

        for _ in 0..system_params.user.user_amount {
            generated_users.push(Point3D {
                x: rng.gen_range(0.0..system_params.area.width),
                y: rng.gen_range(0.0..system_params.area.height),
                z: 0.0, // Ground users stay at altitude 0
            });
        }
        user_points = generated_users;
        println!("✅ Generated {} Users.", user_points.len());
    } else {
        let initial_users = config::load_initial_users("config/initial_users.json");
        println!("✅ Loaded {} Ground Users from JSON.", initial_users.len());
        user_points = initial_users
            .iter()
            .map(|u| Point3D {
                x: u.x,
                y: u.y,
                z: 0.0,
            })
            .collect();
        println!("✅ Loaded {} Users.", initial_users.len());
    }

    let strategy = KMeansPlacement {
        max_iterations: 100,
        target_altitude: 50.0, // UAVs will hover at Z = 50
    };

    println!("\n⚙️  Calculating optimal UAV placement via K-Means...");
    let uav_starting_positions = strategy.generate_positions(
        system_params.uav_amount,
        &user_points,
        &bounds, // Need to map this to Bounds3D first
    );

    println!("\n🎯 Final UAV Starting Positions:");
    for (index, pos) in uav_starting_positions.iter().enumerate() {
        // Using {:.2} to format the floating point numbers to 2 decimal places for readability
        println!(
            "   UAV [{:02}]: X = {:>7.2}, Y = {:>7.2}, Z = {:>5.2}",
            index + 1,
            pos.x,
            pos.y,
            pos.z
        );
    }

    // 5. Initialize and Run ILP Association Strategy
    println!("\n🔗 Calculating optimal ILP User-UAV Association...");
    let association_strategy = IlpAssociation;

    // We pass the calculated UAV positions and the raw user points into the solver
    match association_strategy.associate(
        &user_points,
        &uav_starting_positions,
        system_params.uav_connection_limit,
    ) {
        Ok(association_matrix) => {
            println!("✅ Association successful!");
            for (user_idx, uav_links) in association_matrix.iter().enumerate() {
                for (uav_idx, &is_connected) in uav_links.iter().enumerate() {
                    if is_connected {
                        // User IDs often start at 1 in configs, so we use the actual config ID for clarity
                        println!(
                            "   User [{:02}] -> connected to -> UAV [{:02}]",
                            user_idx + 1,
                            uav_idx + 1
                        );
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Association failed: {}", e);
        }
    }
}
