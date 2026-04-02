use serde::Deserialize;
use std::fs;
use std::path::Path;

// ==========================================
// 1. System Parameters (Parsed from TOML)
// ==========================================
#[derive(Debug, Deserialize)]
pub struct SystemParams {
    pub uav_amount: usize,
    pub bandwidth_user_uav_mbps: f64,
    pub bandwidth_uav_uav_mbps: f64,
    pub uav_computation_capacity_ghz: f64,
    pub uav_speed_m_s: f64,
    pub uav_connection_limit: usize,
    pub area: AreaConfig,
    pub energy_usage: EnergyUsageConfig,
    pub task_profile: TaskProfileConfig,
}

#[derive(Debug, Deserialize)]
pub struct AreaConfig {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
pub struct EnergyUsageConfig {
    pub transmission: f64,
    pub computation: f64,
    // 'move' is a reserved keyword in Rust, so we rename it during parsing
    #[serde(rename = "move")]
    pub movement: f64,
    pub stall: f64,
}

#[derive(Debug, Deserialize)]
pub struct TaskProfileConfig {
    pub required_cycles: u64,
    pub data_size_mb: f64,
    pub requests_per_second: f64,
}

// ==========================================
// 2. Constraints (Parsed from TOML)
// ==========================================
#[derive(Debug, Deserialize)]
pub struct Constraints {
    pub max_delay_tolerance_seconds: f64,
    pub min_overall_success_rate: f64,
}

// ==========================================
// 3. Initial Users (Parsed from JSON)
// ==========================================
#[derive(Debug, Deserialize)]
pub struct InitialUser {
    pub id: u32,
    pub x: f64,
    pub y: f64,
}

pub fn load_system_params<P: AsRef<Path>>(path: P) -> SystemParams {
    let content =
        fs::read_to_string(path).expect("Failed to read system_params.toml. Does the file exist?");
    toml::from_str(&content)
        .expect("Failed to parse system_params.toml. Check for formatting errors.")
}

pub fn load_constraints<P: AsRef<Path>>(path: P) -> Constraints {
    let content = fs::read_to_string(path).expect("Failed to read constraints.toml.");
    toml::from_str(&content).expect("Failed to parse constraints.toml.")
}

pub fn load_initial_users<P: AsRef<Path>>(path: P) -> Vec<InitialUser> {
    let content = fs::read_to_string(path).expect("Failed to read initial_users.json.");
    serde_json::from_str(&content).expect("Failed to parse initial_users.json.")
}
