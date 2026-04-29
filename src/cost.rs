use crate::{vessel::VesselProfile, state::SeaState};

pub struct CostConfig {
    pub weight_fuel: f64,
    pub weight_time: f64,
}

impl CostConfig {
    pub fn balanced() -> Self {
        Self {
            weight_fuel: 0.6,
            weight_time: 0.4,
        }
    }
}

pub fn traversal_cost(
    vessel: &VesselProfile,
    sea: &SeaState,
    distance_nm: f64,
    _heading: f64,
    config: &CostConfig,
) -> f64 {
    let speed = vessel.speed(sea);
    let time = distance_nm / speed;
    let wave_penalty = 1.0 + 0.15 * sea.hs.powi(2);
    let fuel = vessel.fuel_rate(speed) * time * wave_penalty;

    config.weight_fuel * fuel + config.weight_time * time
}