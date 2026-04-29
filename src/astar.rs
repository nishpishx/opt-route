use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

use crate::{grid::Grid, vessel::VesselProfile, cost::{traversal_cost, CostConfig}};

#[derive(Clone)]
struct Node {
    cost: f64,
    r: usize,
    c: usize,
}

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PathResult {
    pub nodes: Vec<(usize, usize)>,
    pub total_cost: f64,
}

pub fn find_path(
    grid: &Grid,
    start: (usize, usize),
    goal: (usize, usize),
    vessel: &VesselProfile,
    config: &CostConfig,
) -> Option<PathResult> {

    let mut pq = BinaryHeap::new();
    let mut dist: HashMap<(usize, usize), f64> = HashMap::new();
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    pq.push(Node { cost: 0.0, r: start.0, c: start.1 });
    dist.insert(start, 0.0);

    while let Some(Node { cost, r, c }) = pq.pop() {

        if (r, c) == goal {
            let mut path = vec![goal];
            let mut cur = goal;

            while let Some(p) = parent.get(&cur) {
                path.push(*p);
                cur = *p;
            }

            path.reverse();

            return Some(PathResult {
                nodes: path,
                total_cost: cost,
            });
        }

        for (nr, nc, dist_step) in grid.neighbors(r, c) {
            let sea = &grid.get(nr, nc).sea;

            let step_cost = traversal_cost(
                vessel,
                sea,
                dist_step,
                0.0,
            );

            let new_cost = cost + step_cost;

            if new_cost < *dist.get(&(nr, nc)).unwrap_or(&f64::INFINITY) {
                dist.insert((nr, nc), new_cost);
                parent.insert((nr, nc), (r, c));

                pq.push(Node {
                    cost: new_cost,
                    r: nr,
                    c: nc,
                });
            }
        }
    }

    None
}