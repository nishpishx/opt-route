use crate::grid::{Grid, Cell};
use crate::state::SeaState;
use std::fs;

pub fn load_grid(path: &str) -> Grid {
    let data = fs::read_to_string(path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&data).unwrap();

    let rows = json["rows"].as_u64().unwrap() as usize;
    let cols = json["cols"].as_u64().unwrap() as usize;

    let mut cells = vec![];

    for r in 0..rows {
        for c in 0..cols {
            let sea = &json["sea_states"][r][c];

            cells.push(Cell {
                sea: SeaState {
                    hs: sea["significant_wave_height_m"].as_f64().unwrap(),
                    mean_dir: sea["mean_direction_deg"].as_f64().unwrap(),
                    steepness: sea["steepness"].as_f64().unwrap(),
                },
                is_land: false,
            });
        }
    }

    Grid { rows, cols, cells }
}