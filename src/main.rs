pub mod config;

fn main() {
    println!("Loading Configuration...");

    let system_params = config::load_system_params("config/system_params.toml");

    println!("✅ Loaded {} UAVs.", system_params.uav_amount);
}
