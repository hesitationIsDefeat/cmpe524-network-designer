//! Orchestrates the main event-driven simulation loop.

use crate::config::{Constraints, SystemParams};
use cmpe524_network_designer::algorithms::offloading::{
    EnergyBalancingOffloader, OffloadDecision, OffloadingStrategy,
};
use cmpe524_network_designer::models::metrics::SimulationMetrics;
use cmpe524_network_designer::models::node::{Task, Uav, User};
use rand_distr::{Distribution, Exp};

pub fn run_event_driven_simulation(
    system_params: &SystemParams,
    constraints: &Constraints,
    users: &mut [User],
    uavs: &mut [Uav],
) -> SimulationMetrics {
    let mut metrics = SimulationMetrics::new();
    let offloader = EnergyBalancingOffloader;
    let mut rng = rand::thread_rng();

    // 1. Pre-generate all tasks (Poisson Arrival Process approximation)
    println!(
        "⏳ Pre-generating tasks for {} seconds of simulation...",
        system_params.simulation.simulation_duration
    );
    let mut all_events: Vec<Task> = Vec::new();
    let mut current_task_id = 0;

    let lambda = system_params.task_profile.requests_per_second;
    let exp_dist = Exp::new(lambda).expect("Requests per second (lambda) must be greater than 0");

    for user in users.iter() {
        let mut current_time = 0.0;

        while current_time < system_params.simulation.simulation_duration {
            // Sample the inter-arrival time safely using the mathematical distribution
            let inter_arrival_time = exp_dist.sample(&mut rng);
            current_time += inter_arrival_time;

            if current_time < system_params.simulation.simulation_duration {
                all_events.push(user.generate_task(
                    current_task_id,
                    current_time,
                    system_params.task_profile.data_size_mb,
                    system_params.task_profile.required_cycles,
                    constraints.max_delay_tolerance_seconds,
                ));
                current_task_id += 1;
            }
        }
    }

    // Sort all tasks chronologically to create our Event Timeline
    all_events.sort_by(|a, b| {
        a.generated_at_time
            .partial_cmp(&b.generated_at_time)
            .unwrap()
    });
    println!(
        "✅ Generated {} total tasks across all users.",
        all_events.len()
    );

    // 2. Main Event Loop
    println!("🚀 Starting Event-Driven Execution...");

    for mut task in all_events {
        let current_time = task.generated_at_time;

        // Step A: Drain the Queues
        // Remove any tasks from UAV queues that finished before this new task arrived
        for uav in uavs.iter_mut() {
            uav.task_queue.retain(|t| {
                t.completed_at_time
                    .expect("Queued task missing completion time")
                    > current_time
            });
        }

        // Step B: Make the Offloading Decision
        let user = &users[task.user_id - 1]; // Assuming IDs are 1-indexed based on previous code

        let decision = offloader.decide(
            &task,
            user,
            uavs,
            system_params.bandwidth_user_uav_mbps,
            system_params.bandwidth_uav_uav_mbps,
        );

        // Step C: Execute the Decision and Update State
        match decision {
            OffloadDecision::Drop => {
                metrics.record_drop();
            }
            OffloadDecision::PrimaryUav { delay } => {
                metrics.record_success(delay);
                task.completed_at_time = Some(current_time + delay);

                let uav_idx = user.connected_uav_id.unwrap() - 1;
                uavs[uav_idx].task_queue.push(task);

                // Deduct Energy (simplified: combining transmission & compute costs)
                uavs[uav_idx].energy_consumed += system_params.energy_usage.computation
                    + system_params.energy_usage.transmission;
            }
            OffloadDecision::ForwardedUav {
                target_uav_id,
                delay,
            } => {
                metrics.record_success(delay);
                task.completed_at_time = Some(current_time + delay);

                // Add to the target UAV's queue
                let target_idx = target_uav_id - 1;
                uavs[target_idx].task_queue.push(task);

                // Deduct Energy from Target UAV (Compute)
                uavs[target_idx].energy_consumed += system_params.energy_usage.computation;

                // Deduct Energy from Primary UAV (Forwarding Transmission)
                let primary_idx = user.connected_uav_id.unwrap() - 1;
                uavs[primary_idx].energy_consumed += system_params.energy_usage.transmission;
            }
        }
    }

    metrics
}
