mod grid;
mod astar;
mod cost;
mod vessel;
mod state;
mod io;

use io::load_grid;
use astar::find_path;
use vessel::VesselProfile;
use cost::CostConfig;

fn main() {
    let grid = load_grid("data/cost_grid.json");

    let vessel = VesselProfile::example();
    let config = CostConfig::balanced();

    let start = (0, 0);
    let goal = (grid.rows - 1, grid.cols - 1);

    let result = find_path(&grid, start, goal, &vessel, &config);

    match result {
        Some(path) => {
            println!("Route found!");
            println!("Total cost: {}", path.total_cost);
            println!("Steps: {}", path.nodes.len());
        }
        None => println!("No route found"),
    }
}