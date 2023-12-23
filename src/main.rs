use std::vec;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::WindowResolution;

use rustilples::cursor::CursorPlugin;
use rustilples::fps::FpsPlugin;
pub use rustilples::world_info::print_world_info;
use rustilples::world_info::WorldInfoPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1000., 800.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FpsPlugin)
        .add_plugins(WorldInfoPlugin)
        .add_systems(Startup, spawn_camera)
        .add_plugins(CursorPlugin)
        .add_systems(Startup, generate_grid)
        .add_systems(Update, grid_click)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum GraphNodeType {
    Standard,
    Blocked,
    RouteHead,
    RoutePoint,
}

#[derive(Component, Debug)]
pub struct GraphNode {
    pub row: u32,
    pub col: u32,
    pub node_type: GraphNodeType,
    pub connections: Vec<(u32, u32)>,
}

pub const GRID_CELL_WIDTH: f32 = 50.0 as f32;
pub const HALF_GRID_CELL_WIDTH: f32 = 25.0 as f32;

pub fn generate_grid(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let width_in_cells = (window.width() / GRID_CELL_WIDTH) as u32;
    let height_in_cells = (window.height() / GRID_CELL_WIDTH) as u32;

    let mut col_index = 0u32;
    let mut row_index = 0u32;
    loop {
        // println!("{}, {}", col_index, row_index);
        if row_index == height_in_cells && col_index == 0 {
            break;
        }

        let x = HALF_GRID_CELL_WIDTH + GRID_CELL_WIDTH * col_index as f32;
        let y = HALF_GRID_CELL_WIDTH + GRID_CELL_WIDTH * row_index as f32;

        // let r = rand::thread_rng().gen_range(0.0..0.2);
        // let g = rand::thread_rng().gen_range(0.4..0.5);
        // let b = rand::thread_rng().gen_range(0.0..0.2);
        // let color = Color::rgb(r, g, b);
        let random_num: u16 = rand::thread_rng().gen_range(1..50000);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: if random_num == 1 {
                        Color::ORANGE
                    } else {
                        Color::GRAY
                    },
                    custom_size: Some(Vec2::new(GRID_CELL_WIDTH, GRID_CELL_WIDTH)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, -1.)),
                ..default()
            },
            GraphNode {
                row: row_index,
                col: col_index,
                connections: vec![],
                node_type: if random_num == 1 {
                    GraphNodeType::Blocked
                } else {
                    GraphNodeType::Standard
                },
            },
        ));

        col_index += 1;
        if col_index == width_in_cells {
            col_index = 0;
            row_index += 1;
        };
    }
}

pub fn grid_click(
    mouse: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut game_grid_nodes: Query<(&mut Sprite, &mut GraphNode), With<GraphNode>>,
) {
    let window = windows.single();

    if mouse.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera.single();

        if let Some(position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            println!("{:?}", position);

            let col_index = ((position.x) / GRID_CELL_WIDTH).floor() as u32;
            let row_index = ((position.y) / GRID_CELL_WIDTH).floor() as u32;
            println!("{}, {}", row_index, col_index);

            // println!("{:?}", game_grid_nodes);

            // for n in game_grid_nodes.iter() {
            //     println!("{:?}", n);
            // }

            if let Some((mut sprite, mut node)) = game_grid_nodes
                .iter_mut()
                .find(|(_, ref node)| node.row == row_index && node.col == col_index)
            {
                if node.node_type != GraphNodeType::Standard {
                    return;
                }
                println!("{:?}", sprite);
                sprite.color = Color::GREEN;
                node.node_type = GraphNodeType::RouteHead;
                println!("{:?}", node);
            };

            // let Some((mut sprite, mut node)) =
            // let mut heads_count = 0;
            // for (_, node) in game_grid_nodes.iter() {
            //     if node.node_type == GraphNodeType::RouteHead { heads_count += 1; }
            // }

            // let mut heads_count = 0;
            let mut route_heads = vec![];
            for (_, node) in game_grid_nodes.iter() {
                if node.node_type == GraphNodeType::RouteHead {
                    route_heads.push((node.col, node.row));
                    // heads_count += 1;
                }
            }

            if route_heads.len() > 1 {
                find_route(
                    route_heads[0],
                    route_heads[1],
                    &game_grid_nodes.iter().map(|(_, node)| node).collect(),
                );
            }
        }
    }
}

// use bevy::ecs::query::QueryIter;

pub fn find_route(start: (u32, u32), end: (u32, u32), game_grid_nodes: &Vec<&GraphNode>) {
    // let mut routes : Vec<(f32, Vec<GraphNode>)> = vec![];

    // let nodes: Vec<&GraphNode> = game_grid_nodes.map(|(_, node)| node).collect();

    // println!("{:?}", game_grid_nodes);

    // let mut traces =
    // let mut traces: Vec<Vec<(u32, u32)>> = vec![];

    // traverse(&start, &end, vec![], game_grid_nodes, &mut vec![], &mut traces);
    // println!("{:?}", r);

    // r.dedup();

    // for i in traces.iter() {
    //     println!("{:?}", i);
    // }
    // println!("len: {:?}", traces.len());

    // let mut visited: Vec<(u32, u32)> = vec![];

    // let r = traverse(&start, &end, game_grid_nodes, &mut vec![], &mut vec![]);
    // println!("result: {:?}", r);
    let r = traverse(&start, &end, game_grid_nodes);
    println!("result: {:?}", r);

}


use std::collections::HashMap;
// (col, row)
pub fn traverse(
    start_node: &(u32, u32),
    end_node: &(u32, u32),
    game_grid_nodes: &Vec<&GraphNode>,
) -> Option<Vec<(u32, u32)>> {
    let mut open_set: Vec<(u32, u32)> = vec![];
    open_set.push(*start_node);

    let mut came_from: HashMap<(u32, u32), (u32, u32)> = HashMap::new();
    let mut g_score: HashMap<(u32, u32), f32> = HashMap::new();
    g_score.insert(*start_node, 0.0);


    let mut f_score: HashMap<(u32, u32), f32> = HashMap::new();
    f_score.insert(*start_node, 0.0);

    while open_set.len() > 0 {
        let mut current = open_set[0];

        for node in open_set.iter() {
            if let Some(score) = f_score.get(node) {
                if score < f_score.get(&current).unwrap() {
                    current = *node;
                }
            }
        }

        if &current == end_node {
            println!(
                "------- A* score: {} --------",
                f_score.get(&current).unwrap()
            );

            return Some(reconstruct_path(came_from, end_node));
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
            if game_grid_nodes
                .iter()
                .find(|node| {
                    (node.col == neighbor.0 && node.row == neighbor.1)
                        && (node.node_type == GraphNodeType::Standard
                            || node.node_type == GraphNodeType::RouteHead)
                })
                .is_none()
            {
                continue;
            };

         let tentative_g_score = g_score.get(&current).unwrap() + 1.0;
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


pub fn reconstruct_path(came_from: HashMap<(u32, u32), (u32, u32)>, end: &(u32, u32)) -> Vec<(u32, u32)> {
    let mut total_path = vec![];

    total_path.push(*end);

    let mut current = end;

    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap();

        total_path.insert(0, *current);
    }

    total_path
}





// pub fn traverse(
//     current_node: &(u32, u32),
//     end_node: &(u32, u32),
//     game_grid_nodes: &Vec<&GraphNode>,
//     // mut visited: &mut Vec<(u32, u32)>,
//     mut came_from: &mut Vec<(u32, u32)>,
//     mut cache: &mut Vec<(((u32, u32), (u32, u32)), Option<Vec<(u32, u32)>>)>,
//     // start: &(u32, u32),
//     // end: &(u32, u32),
//     // mut trace: Vec<(u32, u32)>,
//     // game_grid_nodes: &Vec<&GraphNode>,
//     // mut traces: &mut Vec<Vec<(u32, u32)>>,
// ) -> Option<Vec<(u32, u32)>> {

//     let memoized = cached(cache, (*current_node, *end_node), None);
//     if memoized.is_some() { 
//         if let Some(ref cached_val) = memoized {
//             if cached_val.is_empty() { return None; }
//         }
//         return memoized;
//     }


//     // println!("cache miss {:?}, {:?}", current_node, end_node);
//     // println!("cache len {:?}", cache.len());


//     if game_grid_nodes.len() / 3 < came_from.len() {
//         // println!("too long");
//         // println!("{:?}", came_from);
//         return cached(cache, (*current_node, *end_node), None);
//         // return None;
//     };

//     if current_node == end_node {
//         return Some(vec![*end_node]);
//     }

//     let mut connections: Vec<(u32, u32)> = vec![];

//     connections.push((current_node.0 + 1, current_node.1));
//     if current_node.0 > 0 {
//         connections.push((current_node.0 - 1, current_node.1))
//     };
//     connections.push((current_node.0, current_node.1 + 1));
//     if current_node.1 > 0 {
//         connections.push((current_node.0, current_node.1 - 1))
//     };

//     came_from.push(*current_node);
//     let mut neigbors_shortest_path: Option<Vec<(u32, u32)>> = None;
//     for neighbor in connections.iter() {
//         if came_from.contains(neighbor) {
//             continue;
//         }

//         if game_grid_nodes
//             .iter()
//             .find(|node| {
//                 (node.col == neighbor.0 && node.row == neighbor.1)
//                     && (node.node_type == GraphNodeType::Standard
//                         || node.node_type == GraphNodeType::RouteHead)
//             })
//             .is_none()
//         {
//             continue;
//         };

//         if neighbor == end_node {
//             return Some(vec![*current_node, *neighbor]);
//         }




//         let mut came_from_copy = came_from.clone();

        
        
        
//         let route = traverse(neighbor, end_node, game_grid_nodes, &mut came_from_copy, cache);
//         let route = cached(cache, (*neighbor, *end_node), route);

//         match route {
//         // match traverse(neighbor, end_node, game_grid_nodes, &mut came_from_copy, cache) {
//             None => {}
//             Some(mut path) => match neigbors_shortest_path {
//                 None => {
//                     // println!("no prev nei route: {:?}", path);

//                     let mut current_path = vec![*current_node];
//                     current_path.append(&mut path);

//                     // println!("res: {:?}", current_path);

                     
//                      neigbors_shortest_path = cached(cache, (*current_node, *end_node), Some(current_path));
//                 }
//                 Some(ref current_shortest) => {
//                     cached(cache, (*neighbor, *end_node), Some(path.clone()));

//                     if current_shortest.len() > path.len() {
//                         let mut current_path = vec![*current_node];
//                         current_path.append(&mut path);
//                         neigbors_shortest_path = Some(current_path);
//                     }
//                 }
//             },
//         }

//         // neigbors_shortest_path = cached(cache, (*neighbor, *end_node), neigbors_shortest_path);
//     }

//     neigbors_shortest_path
// }





// cached(cache, ((), ()), Some(vec![]));
// pub fn cached(
//     mut cache: &mut Vec<(((u32, u32), (u32, u32)), Option<Vec<(u32, u32)>>)>,
//     key: ((u32, u32), (u32, u32)),
//     path: Option<Vec<(u32, u32)>>,
// ) -> Option<Vec<(u32, u32)>> {
//     println!("cache key {:?}, {:?}", key, path);


//     if let Some((key, val)) = cache.iter().find(|(address, _)| *address == key) {
//         match val {
//             Some(cached) =>{
//                 match path {
//                     None => { return Some(vec![]) },
//                     Some(ref nodes) => {
//                         if cached.len() <= nodes.len() {
//                             println!("cache hit");
                            
//                             return val.clone();
//                         } else {
//                             cache.push((*key, path.clone()));
//                             cache.dedup_by(|b, a| a.0 == b.0);
//                             return path;
//                         }
//                     }
//                 }
                
//             },
//             None => {
//                 cache.push((*key, path.clone()));
//                 cache.dedup_by(|b, a| a.0 == b.0);
//                 return path;
//             }
//         }
//     } else {
//         cache.push((key, path.clone()));
//         cache.dedup_by(|b, a| a.0 == b.0);
//         return path;
//     }
// }

// if trace.contains(start) {
//     cache.push(((trace[0], *start), vec![]));
//     return;
// } else {
//     trace.push(*start);
// };
// let trace_len = trace.len();
// let max_len = game_grid_nodes.len() /3;
// if trace_len > max_len {
//     cache.push(((trace[0], *start), vec![]));

//      return;
// }

// let shortest_len = match  traces
// .iter()
// .min_by(|a, b| a.len().cmp(&b.len())) {
//     None => usize::MAX,
//     Some(shortest) => shortest.len()
// };

// if shortest_len <= trace_len {
//     cache.push(((trace[0], *start), vec![]));

//     return;
// }

// let mut connections: Vec<(u32, u32)> = vec![];

// connections.push((start.0 + 1, start.1));
// if start.0 > 0 {
//     connections.push((start.0 - 1, start.1))
// };
// connections.push((start.0, start.1 + 1));
// if start.1 > 0 {
//     connections.push((start.0, start.1 - 1))
// };

// // println!("{:?}", connections);
// for connection in connections.iter() {
//     if trace.contains(connection) {
//         // println!("{}", 99999);
//         cache.push(((trace[0], *connection), vec![]));

//         continue;
//     };

//     let available_node = game_grid_nodes.iter().find(|node| {
//         (node.col == connection.0 && node.row == connection.1)
//             && (node.node_type == GraphNodeType::Standard
//                 || node.node_type == GraphNodeType::RouteHead)
//     });
//     if available_node.is_none() {
//         // println!("{}", 111111111);
//         cache.push(((trace[0], *connection), vec![]));

//         continue;
//     };

//     if let Some((key, cached)) = cache.iter().find(|(address, _)| *address == (trace[0], *connection)) {
//         if cached.len() <= trace_len {
//             return;
//         } else {
//             cache.push(((trace[0], *connection), trace.clone()));
//         }
//     } else {
//         cache.push(((trace[0], *connection), trace.clone()));
//     }

//     if connection == end {
//         trace.push(*connection);

//         if shortest_len > trace.len() {
//             traces.push(trace.clone());
//         }
//     } else {
//         let mut trace_copy = trace.clone();
//         traverse(&connection, &end, trace_copy, game_grid_nodes, cache, traces);
//     }

//     }

// }

// // (col, row)
// pub fn traverse(
//     start: &(u32, u32),
//     end: &(u32, u32),
//     mut trace: Vec<(u32, u32)>,
//     game_grid_nodes: &Vec<&GraphNode>,
//     mut cache: &mut Vec<(((u32, u32), (u32, u32)), Vec<(u32, u32)>)>,
//     mut traces: &mut Vec<Vec<(u32, u32)>>,
// )  {
//     if trace.contains(start) {
//         cache.push(((trace[0], *start), vec![]));
//         return;
//     } else {
//         trace.push(*start);
//     };
//     let trace_len = trace.len();
//     let max_len = game_grid_nodes.len() /3;
//     if trace_len > max_len {
//         cache.push(((trace[0], *start), vec![]));

//          return;
//     }

//     let shortest_len = match  traces
//     .iter()
//     .min_by(|a, b| a.len().cmp(&b.len())) {
//         None => usize::MAX,
//         Some(shortest) => shortest.len()
//     };

//     if shortest_len <= trace_len {
//         cache.push(((trace[0], *start), vec![]));

//         return;
//     }

//     let mut connections: Vec<(u32, u32)> = vec![];

//     connections.push((start.0 + 1, start.1));
//     if start.0 > 0 {
//         connections.push((start.0 - 1, start.1))
//     };
//     connections.push((start.0, start.1 + 1));
//     if start.1 > 0 {
//         connections.push((start.0, start.1 - 1))
//     };

//     // println!("{:?}", connections);
//     for connection in connections.iter() {
//         if trace.contains(connection) {
//             // println!("{}", 99999);
//             cache.push(((trace[0], *connection), vec![]));

//             continue;
//         };

//         let available_node = game_grid_nodes.iter().find(|node| {
//             (node.col == connection.0 && node.row == connection.1)
//                 && (node.node_type == GraphNodeType::Standard
//                     || node.node_type == GraphNodeType::RouteHead)
//         });
//         if available_node.is_none() {
//             // println!("{}", 111111111);
//             cache.push(((trace[0], *connection), vec![]));

//             continue;
//         };

//         if let Some((key, cached)) = cache.iter().find(|(address, _)| *address == (trace[0], *connection)) {
//             if cached.len() <= trace_len {
//                 return;
//             } else {
//                 cache.push(((trace[0], *connection), trace.clone()));
//             }
//         } else {
//             cache.push(((trace[0], *connection), trace.clone()));
//         }

//         if connection == end {
//             trace.push(*connection);

//             if shortest_len > trace.len() {
//                 traces.push(trace.clone());
//             }
//         } else {
//             let mut trace_copy = trace.clone();
//             traverse(&connection, &end, trace_copy, game_grid_nodes, cache, traces);
//         }

//     }

// }

// // (col, row)
// pub fn traverse(
//     start: &(u32, u32),
//     end: &(u32, u32),
//     mut trace: Vec<(u32, u32)>,
//     game_grid_nodes: &Vec<&GraphNode>,
// ) -> Vec<Vec<(u32, u32)>> {
//     let mut traces: Vec<Vec<(u32, u32)>> = vec![];
//     let mut connections: Vec<(u32, u32)> = vec![];

//     connections.push((start.0 + 1, start.1));
//     if start.0 > 0 {
//         connections.push((start.0 - 1, start.1))
//     };
//     connections.push((start.0, start.1 + 1));
//     if start.1 > 0 {
//         connections.push((start.0, start.1 - 1))
//     };
//     // println!("{:?}", start);

//     for connection in connections.iter() {
//         if trace.contains(connection) {
//             continue;
//         };

//         let available_node = game_grid_nodes.iter().find(|node| {
//             (node.col == connection.0 && node.row == connection.1)
//                 && (node.node_type == GraphNodeType::Standard
//                     || node.node_type == GraphNodeType::RouteHead)
//         });
//         if available_node.is_none() {
//             println!("{}", 111111111);
//             continue;
//         };

//         // if (connection == end) {
//         if (connection == end) {
//             // if (connection.0 == end.0 && connection.1 == end.1) {
//             trace.push(*start);
//             trace.push(*connection);
//             traces.push(trace.clone());
//             println!("{}", 222222222);
//         } else if end == start {
//             println!("{}", 444444);
//             continue;
//         }else {
//             println!("{}", 333333333);

//             let mut new_trace = trace.clone();
//             // new_trace.push(*start);
//             new_trace.push(*connection);

//             // (|x, y| x.cmp(y)).unwrap(), 5);
//             if let Some(shortest) = traces
//                 .iter()
//                 .map(|item| item.len())
//                 .min_by(|a, b| a.cmp(b))
//             {

//                 if new_trace.len() > shortest {
//                     // println!("{} > {}", new_trace.len(), shortest);
//                     continue;
//                 }
//             }

//             let mut path_results = traverse(&connection, end, new_trace, &game_grid_nodes);

//             traces.append(&mut path_results);
//         }
//     }
//     traces
// }

// #[derive(Component, Debug)]
// pub struct Graph {
//     pub store: Vec<Connection>,
// }

// #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
// pub struct Connection {
//     pub cost: f32,
//     pub a_node: GraphNode,
//     pub b_node: GraphNode,
// }

// #[derive(Component, Debug, Clone)]
// pub struct Path {
//     pub connections: Vec<Connection>,
// }

// impl Path {
//     pub fn cost(&self) -> f32 {
//         let mut sum = 0.0;
//         for connection in &self.connections {
//             sum += connection.cost;
//         }
//         sum
//     }
// }

// // #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
// // pub struct Node {
// //     pub id: u32,
// // }

// // Directional, weighted graph
// impl Graph {
//     pub fn get_connections(&self, from_node: &GraphNode) -> Vec<&Connection> {
//         let mut result = Vec::new();

//         for connection in self.store.iter().filter(|item| &item.a_node == from_node) {
//             result.push(connection);
//         }

//         result
//     }
// }

// pub fn dijkstra_search(
//     graph: &Graph,
//     start: &GraphNode,
//     end: &GraphNode,
//     trace: Vec<Connection>,
// ) -> Option<Path> {
//     let pathes = traverse(graph, &start, end, trace);
//     if pathes.len() > 0 {
//         let cheapest = pathes
//             .iter()
//             .min_by(|a, b| a.cost().partial_cmp(&b.cost()).unwrap())
//             .unwrap();

//         Some(cheapest.clone())
//     } else {
//         None
//     }
// }

// pub fn traverse(graph: &Graph, start: &GraphNode, end: &GraphNode, mut trace: Vec<Connection>) -> Vec<Path> {
//     let mut traces = Vec::new();
//     let connections = graph.get_connections(&start);

//     for connection in connections.iter() {
//         if (connection.b_node == *end) || connection.a_node == *end {
//             trace.push(**connection);

//             traces.push(Path {
//                 connections: trace.clone(),
//             });
//         } else {
//             let mut new_trace = trace.clone();
//             new_trace.push(**connection);

//             let mut path_results = traverse(graph, &connection.b_node, end, new_trace);

//             traces.append(&mut path_results);
//         }
//     }

//     traces
// }

// use bevy::sprite::MaterialMesh2dBundle;

// pub const HERO_SPEED: f32 = 500.0;
// pub const HERO_SIZE: f32 = 50.0;

//     .add_systems(Startup, spawn_camera)
//     .add_systems(Startup, spawn_castle)
//     .add_systems(Startup, spawn_hero)
//     .add_systems(Update, hero_movement)
//     .add_systems(Update, confine_hero_movement)
// // .add_startup_system(spawn_hero)
// .run();
//     .add_systems(Startup, setup)
//     .add_systems(Update, mouse_button_iter)
//     .run();

// #[derive(Component)]
// pub struct Hero;

// #[derive(Component, Debug)]
// pub struct Castle {
//     width: f32,
//     height: f32
// }

// #[derive(Component)]
// struct Name(String);

// pub fn spawn_castle(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let window = window_query.get_single().unwrap();
//     let transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
//                                          .with_scale(Vec3 { x: 100, y200 0.4, z: 0.4 });

//     commands.spawn((
//         SpriteBundle {
//             transform: transform,
//             texture: asset_server.load("sprites/castle.png"),
//             ..default()
//         },
//         Castle {
//             width: 354.0 * 0.4,
//             height: 296.0 * 0.4
//         }
//     ));

//     let transform = Transform::from_xyz( 100.0, window.height() / 3.0, 0.0)
//                                          .with_scale(Vec3 { x: 0.4, y: 0.4, z: 0.4 });

//     commands.spawn((
//         SpriteBundle {
//             transform: transform,
//             texture: asset_server.load("sprites/castle2.png"),
//             ..default()
//         },
//         Castle {
//             width: 448.0 * 0.4,
//             height: 340.0 * 0.4
//         }
//     ));

//     let transform = Transform::from_xyz( window.width() - 200.0, window.height() / 3.0, 0.0)
//                                          .with_scale(Vec3 { x: 0.4, y: 0.4, z: 0.4 });

//     commands.spawn((
//         SpriteBundle {
//             transform: transform,
//             texture: asset_server.load("sprites/castle3.png"),
//             ..default()
//         },
//         Castle {
//             width: 353.0 * 0.4,
//             height: 353.0 * 0.4
//         }
//     ));
// }

// pub fn spawn_hero(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let window = window_query.get_single().unwrap();

//     commands.spawn((
//         SpriteBundle {
//             transform: Transform::from_xyz(30.0, 30.0, 0.0),
//             texture: asset_server.load("sprites/hero.png"),
//             ..default()
//         },
//         Hero {},
//     ));
// }

// pub fn hero_movement(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut hero_query: Query<&mut Transform, With<Hero>>,
//     castles_query: Query<(&Castle, &Transform), Without<Hero>>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     time: Res<Time>,
// ) {
//     if let Ok(mut hero_transform) = hero_query.get_single_mut() {

//         // if let Ok((hero_entity, mut hero_transform)) = hero_query.get_single_mut() {

//         // }

//         let mut direction = Vec3::ZERO;

//         if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
//             direction += Vec3::new(-1.0, 0.0, 0.0);
//         }
//         if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
//             direction += Vec3::new(1.0, 0.0, 0.0);
//         }
//         if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
//             direction += Vec3::new(0.0, 1.0, 0.0);
//         }
//         if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
//             direction += Vec3::new(0.0, -1.0, 0.0);
//         }

//         if direction.length() > 0.0 {
//             direction = direction.normalize();
//         }

//         let result_direction = direction * HERO_SPEED * time.delta_seconds();
//         let window = window_query.get_single().unwrap();

//             let half_hero_size = HERO_SIZE / 2.0; // 32.0
//             let x_min = 0.0 + half_hero_size;
//             let x_max = window.width() - half_hero_size;
//             let y_min = 0.0 + half_hero_size;
//             let y_max = window.height() - half_hero_size;

//             let mut translation = hero_transform.translation;

//             // Bound the hero x position
//             if translation.x < x_min {
//                 return
//             } else if translation.x > x_max {
//                 return
//             }
//             // Bound the heros y position.
//             if translation.y < y_min {
//                 return
//             } else if translation.y > y_max {
//                 return
//             }

//             let h_max_x = hero_transform.translation.x + half_hero_size + result_direction.x;
//             let h_min_x = hero_transform.translation.x - half_hero_size - result_direction.x;
//             let h_max_y = hero_transform.translation.y + half_hero_size - result_direction.y;
//             let h_min_y = hero_transform.translation.y - half_hero_size + result_direction.y;

//             for (castle, castle_transform) in castles_query.iter() {
//                     let c_max_x = castle_transform.translation.x + castle.width / 2.0;
//                     let c_min_x = castle_transform.translation.x - castle.width / 2.0;
//                     let c_max_y = castle_transform.translation.y + castle.height / 2.0;
//                     let c_min_y = castle_transform.translation.y - castle.height / 2.0;

//                     if h_min_y < c_max_y && h_max_y > c_min_y {
//                         if h_max_x > c_min_x && h_min_x < c_max_x  {
//                             return
//                        }
//                     }
//             }

//         hero_transform.translation += result_direction;
//     }
// }

// pub fn confine_hero_movement(
//     mut hero_query: Query<(&Hero, &mut Transform), Without<Castle>>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     castles_query: Query<(&Castle, &Transform), Without<Hero>>,
// ) {
//     // if let Ok((hero_entity, mut hero_transform)) = hero_query.get_single_mut() {
//     //     let window = window_query.get_single().unwrap();

//     //     let half_hero_size = HERO_SIZE / 2.0; // 32.0
//     //     let x_min = 0.0 + half_hero_size;
//     //     let x_max = window.width() - half_hero_size;
//     //     let y_min = 0.0 + half_hero_size;
//     //     let y_max = window.height() - half_hero_size;

//     //     let mut translation = hero_transform.translation;

//     //     // Bound the hero x position
//     //     if translation.x < x_min {
//     //         return
//     //     } else if translation.x > x_max {
//     //         return
//     //     }
//     //     // Bound the heros y position.
//     //     if translation.y < y_min {
//     //         return
//     //     } else if translation.y > y_max {
//     //         return
//     //     }

//     //     let h_max_x = hero_transform.translation.x + half_hero_size;
//     //     let h_min_x = hero_transform.translation.x - half_hero_size;
//     //     let h_max_y = hero_transform.translation.y + half_hero_size;
//     //     let h_min_y = hero_transform.translation.y - half_hero_size;

//     //     for (castle, castle_transform) in castles_query.iter() {
//     //             let c_max_x = castle_transform.translation.x + castle.width / 2.0;
//     //             let c_min_x = castle_transform.translation.x - castle.width / 2.0;
//     //             let c_max_y = castle_transform.translation.y + castle.height / 2.0;
//     //             let c_min_y = castle_transform.translation.y - castle.height / 2.0;

//     //             if h_min_y < c_max_y && h_max_y > c_min_y {
//     //                 if h_max_x > c_min_x && h_min_x < c_max_x  {
//     //                     return
//     //                }
//     //             }
//     //     }

//     // }
// }

// // fn spawn_castles(mut commands: Commands) {
// //     commands.spawn((Castle, Name("Elaina".to_string())));
// //     commands.spawn((Castle, Name("Renzo".to_string())));
// //     commands.spawn((Castle, Name("Zayna".to_string())));
// // }

// // fn render_castles(query: Query<&Name, With<Castle>>) {
// //     for name in &query {
// //         println!("castle {}", name.0);
// //     }
// // }

// // fn mouse_button_iter(
// //     buttons: Res<Input<MouseButton>>,
// // ) {
// //     for button in buttons.get_pressed() {
// //         println!("{:?} is currently held down", button);
// //     }
// //     for button in buttons.get_just_pressed() {
// //         println!("{:?} was pressed", button);
// //     }
// //     for button in buttons.get_just_released() {
// //         println!("{:?} was released", button);
// //     }
// // }

// // fn cursor_position(
// //     q_windows: Query<&Window, With<PrimaryWindow>>,
// // ) {
// //     // Games typically only have one window (the primary window)
// //     if let Some(position) = q_windows.single().cursor_position() {
// //         println!("Cursor is inside the primary window, at {:?}", position);
// //     } else {
// //         println!("Cursor is not in the game window.");
// //     }
// // }

// // fn setup(
// //     mut commands: Commands,
// //     mut meshes: ResMut<Assets<Mesh>>,
// //     mut materials: ResMut<Assets<ColorMaterial>>,
// // ) {
// //     commands.spawn(Camera2dBundle::default());

// //     // Circle
// //     commands.spawn(MaterialMesh2dBundle {
// //         mesh: meshes.add(shape::Circle::new(50.).into()).into(),
// //         material: materials.add(ColorMaterial::from(Color::PURPLE)),
// //         transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
// //         ..default()
// //     });

// //     // Rectangle
// //     commands.spawn(SpriteBundle {
// //         sprite: Sprite {
// //             color: Color::rgb(0.25, 0.25, 0.75),
// //             custom_size: Some(Vec2::new(50.0, 100.0)),
// //             ..default()
// //         },
// //         transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
// //         ..default()
// //     });

// //     // Quad
// //     commands.spawn(MaterialMesh2dBundle {
// //         mesh: meshes
// //             .add(shape::Quad::new(Vec2::new(50., 100.)).into())
// //             .into(),
// //         material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
// //         transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
// //         ..default()
// //     });

// //     // Hexagon
// //     commands.spawn(MaterialMesh2dBundle {
// //         mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
// //         material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
// //         transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
// //         ..default()
// //     });
// // }

// // pub struct HelloPlugin;

// // impl Plugin for HelloPlugin {
// //     fn build(&self, app: &mut App) {
// //         println!("hello world!");
// //     }
// // }

// // pub struct CastlesPlugin;

// // impl Plugin for CastlesPlugin {
// //     fn build(&self, app: &mut App) {
// //         app.add_systems(Startup, add_castles)
// //            .add_systems(Update, render_castles);
// //     }
// // }
