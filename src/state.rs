use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SeaState {
    pub hs: f64,
    pub mean_dir: f64,
    pub steepness: f64,
}