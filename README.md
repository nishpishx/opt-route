# voyager

Optimal maritime route planner written in Rust. Given a grid of sea-state conditions, finds the least-cost path from origin to destination using Dijkstra's algorithm with a fuel+time cost function.

## How it works

The ocean is represented as a 2D grid where each cell holds a `SeaState` (significant wave height, mean wave direction, steepness). The planner searches over this grid with 8-directional movement, computing traversal cost at each step based on the vessel's fuel consumption and the local sea conditions.

**Cost model**

```
traversal_cost = fuel_consumed × wave_penalty
wave_penalty   = 1 + 0.15 × Hs²
fuel_consumed  = base_fuel_tpd × (speed / max_speed)³ / 24 × time
```

The final cost blends fuel (weight 0.6) and time (weight 0.4) via `CostConfig`.

## Input format

Expects a JSON file at `data/cost_grid.json`:

```json
{
  "rows": 10,
  "cols": 10,
  "sea_states": [
    [
      { "significant_wave_height_m": 1.2, "mean_direction_deg": 45.0, "steepness": 0.03 },
      ...
    ],
    ...
  ]
}
```

`sea_states` is a `rows × cols` 2D array (row-major).

## Usage

```bash
cargo run
```

The binary loads `data/cost_grid.json`, runs the planner from `(0, 0)` to the bottom-right corner, and prints the total cost and number of steps.

## Project structure

```
src/
  main.rs      — entry point
  grid.rs      — Grid and Cell types, 8-directional neighbor expansion
  astar.rs     — Dijkstra search, PathResult
  cost.rs      — traversal_cost, CostConfig (fuel/time weights)
  vessel.rs    — VesselProfile (speed, cubic fuel law)
  state.rs     — SeaState (Hs, mean direction, steepness)
  io.rs        — JSON grid loader
```

## Dependencies

- [`serde`](https://crates.io/crates/serde) + `serde_json` — JSON deserialization
- [`priority-queue`](https://crates.io/crates/priority-queue) — priority queue for Dijkstra
