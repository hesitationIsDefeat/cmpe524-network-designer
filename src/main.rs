pub mod config;

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
}
