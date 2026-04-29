use crate::state::SeaState;

pub struct VesselProfile {
    pub max_speed_knots: f64,
    pub base_fuel_tpd: f64,
}

impl VesselProfile {
    pub fn example() -> Self {
        Self {
            max_speed_knots: 18.0,
            base_fuel_tpd: 40.0,
        }
    }

    pub fn speed(&self, _sea: &SeaState) -> f64 {
        self.max_speed_knots // MVP: no physics yet
    }

    pub fn fuel_rate(&self, speed: f64) -> f64 {
        let ratio = speed / self.max_speed_knots;
        self.base_fuel_tpd * ratio.powi(3) / 24.0
    }
}