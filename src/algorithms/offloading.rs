use crate::models::node::{Task, Uav, User};

pub enum OffloadDecision {
    PrimaryUav { delay: f64 },
    ForwardedUav { target_uav_id: usize, delay: f64 },
    Drop,
}

pub trait OffloadingStrategy {
    fn decide(
        &self,
        task: &Task,
        user: &User,
        uavs: &[Uav],
        bw_user_uav_mbps: f64,
        bw_uav_uav_mbps: f64,
    ) -> OffloadDecision;
}

pub struct EnergyBalancingOffloader;

impl OffloadingStrategy for EnergyBalancingOffloader {
    fn decide(
        &self,
        task: &Task,
        user: &User,
        uavs: &[Uav],
        bw_user_uav_mbps: f64,
        bw_uav_uav_mbps: f64,
    ) -> OffloadDecision {
        let primary_uav_id = match user.connected_uav_id {
            Some(id) => id,
            None => return OffloadDecision::Drop, // User has no network connection
        };

        let mut best_candidate: Option<(usize, f64)> = None;
        let mut lowest_energy = f64::MAX;

        for uav in uavs {
            let mut total_delay = 0.0;

            total_delay += task.data_size_mb / bw_user_uav_mbps;

            if uav.base.id != primary_uav_id {
                total_delay += task.data_size_mb / bw_uav_uav_mbps;
            }

            total_delay += uav.estimate_queue_delay();

            if uav.capacity > 0.0 {
                let cycles_per_sec = uav.capacity * 1_000_000_000.0;
                total_delay += task.required_cycles as f64 / cycles_per_sec;
            } else {
                continue;
            }

            if total_delay <= task.max_delay_tolerance_seconds {
                if uav.energy_consumed < lowest_energy {
                    lowest_energy = uav.energy_consumed;
                    best_candidate = Some((uav.base.id, total_delay));
                }
            }
        }

        match best_candidate {
            Some((uav_id, delay)) if uav_id == primary_uav_id => {
                OffloadDecision::PrimaryUav { delay }
            }
            Some((uav_id, delay)) => OffloadDecision::ForwardedUav {
                target_uav_id: uav_id,
                delay,
            },
            None => OffloadDecision::Drop,
        }
    }
}
