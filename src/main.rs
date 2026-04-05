pub mod config;
use cmpe524_network_designer::algorithms::placement::KMeansPlacement;
use cmpe524_network_designer::algorithms::placement::PlacementStrategy;
use cmpe524_network_designer::models::geo::Bounds3D;
use cmpe524_network_designer::models::geo::Point3D;

fn main() {
    println!("Loading Configuration...");

    let system_params = config::load_system_params("config/system_params.toml");
    let constraints = config::load_constraints("config/constraints.toml");
    let initial_users = config::load_initial_users("config/initial_users.json");

    println!("✅ Loaded {} UAVs.", system_params.uav_amount);
    println!(
        "✅ Target Success Rate: {}%",
        constraints.min_overall_success_rate * 100.0
    );
    println!("✅ Loaded {} Users.", initial_users.len());

    let user_points: Vec<Point3D> = initial_users
        .iter()
        .map(|u| Point3D {
            x: u.x,
            y: u.y,
            z: 0.0,
        })
        .collect();

    let bounds: Bounds3D = system_params.area.to_bounds3d();

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
}
