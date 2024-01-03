use crate::game_grid::graph_node::*;
use std::collections::HashMap;
use std::collections::VecDeque;

pub mod action;
pub mod find_path_action;
pub mod pathfinding_params;
pub mod pickup_axe_action;
pub mod state;

use crate::game_grid::ai::action::*;
use crate::game_grid::ai::find_path_action::*;
use crate::game_grid::ai::pathfinding_params::*;
use crate::game_grid::ai::pickup_axe_action::*;
use crate::game_grid::ai::state::*;

// actions:
// - find path [done]
// - pickup axe [done]
// - pickup wood
// - pickup iron
// - craft axe

pub fn plan_path(mut params: PathfindingParams) -> Option<Vec<(u32, u32)>> {
    let mut state = State {
        path: None,
        cost: None,
        actions: VecDeque::new(),
        destination_reached: false,
    };
    state.actions.push_front(Box::new(FindPathAction {}));

    while let Some(action) = state.actions.pop_front() {
        action.exec(&mut params, &mut state);

        if state.destination_reached {
            state.actions.clear();
            return state.path;
        }
    }

    None
}

// (col, row)
pub fn find_path(params: &mut PathfindingParams) -> Option<Vec<(u32, u32)>> {
    let mut open_set: Vec<(u32, u32)> = vec![];
    open_set.push(*params.start_node);

    let mut came_from: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut g_score: HashMap<(u32, u32), f32> = HashMap::new();
    g_score.insert(*params.start_node, 0.0);

    let mut f_score: HashMap<(u32, u32), f32> = HashMap::new();
    f_score.insert(*params.start_node, 0.0);

    while open_set.len() > 0 {
        let mut current = open_set[0];

        for node in open_set.iter() {
            if let Some(score) = f_score.get(node) {
                if score < f_score.get(&current).unwrap() {
                    current = *node;
                }
            }
        }

        if &current == params.end_node {
            println!(
                "------- A* score: {} --------",
                f_score.get(&current).unwrap()
            );

            return Some(reconstruct_path(came_from, params.end_node));
        }

        let remove_index = open_set.iter().position(|item| *item == current).unwrap();
        open_set.remove(remove_index);

        let mut connections: Vec<(u32, u32)> = vec![];

        connections.push((current.0 + 1, current.1));
        if current.0 > 0 {
            connections.push((current.0 - 1, current.1))
        };
        connections.push((current.0, current.1 + 1));
        if current.1 > 0 {
            connections.push((current.0, current.1 - 1))
        };

        for neighbor in connections.iter() {
            let neighbor_node = params.game_grid_nodes.iter().find(|node| {
                (node.col == neighbor.0 && node.row == neighbor.1)
                    && params.graph_node_types.contains(&node.node_type)
            });
            if neighbor_node.is_none() {
                continue;
            };

            let pathing_cost = PATHING_COST[&neighbor_node.unwrap().node_type];

            let tentative_g_score = g_score.get(&current).unwrap() + pathing_cost;
            if &tentative_g_score < g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                // This path to neighbor is better than any previous one. Record it!
                came_from.insert(*neighbor, current.clone());
                g_score.insert(*neighbor, tentative_g_score);

                let f_score_neighbor = tentative_g_score;
                f_score.insert(*neighbor, f_score_neighbor);

                if !open_set.contains(&&neighbor) {
                    open_set.push(*neighbor);
                }
            }
        }
    }

    None
}

pub fn reconstruct_path(
    came_from: HashMap<(u32, u32), (u32, u32)>,
    end: &(u32, u32),
) -> Vec<(u32, u32)> {
    let mut total_path = vec![];

    total_path.push(*end);

    let mut current = end;

    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap();

        total_path.insert(0, *current);
    }

    total_path
}
