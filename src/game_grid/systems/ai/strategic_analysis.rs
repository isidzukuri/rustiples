use super::*;

pub fn find_position_amid(
    params: &PathfindingParams,
    node_type: GridEntityType,
) -> Option<(u32, u32)> {
    let mut node_rates: Vec<((u32, u32), f32)> = vec![];

    for node in params.grid.index().iter() {
        if node.entity_type.is_some() {
            continue;
        }

        let convolution_window = convolution_window_nodes((node.x, node.y), 2);
        let rate = params
            .grid
            .index()
            .iter()
            .filter(|grid_node| {
                grid_node.entity_type == Some(node_type)
                    && convolution_window.contains(&(grid_node.x, grid_node.y))
            })
            .count();

        node_rates.push(((node.x, node.y), (rate as f32)));
    }

    match node_rates.iter().max_by(|a, b| a.1.total_cmp(&b.1)) {
        Some(rate) => return Some(rate.0),
        None => return None,
    }
}

pub fn convolution_window_nodes(node: (u32, u32), size: u32) -> Vec<(u32, u32)> {
    let mut nodes: Vec<(u32, u32)> = vec![node];
    let mut shift = 1;

    while shift <= size {
        nodes.push((node.0 + shift, node.1));
        nodes.push((node.0, node.1 + shift));
        nodes.push((node.0 + shift, node.1 + shift));

        if (node.1 as i32 - shift as i32) > 0 {
            nodes.push((node.0, node.1 - shift));
            nodes.push((node.0 + shift, node.1 - shift));
        };
        if (node.0 as i32 - shift as i32) > 0 {
            nodes.push((node.0 - shift, node.1));
            nodes.push((node.0 - shift, node.1 + shift));
        };
        if (node.1 as i32 - shift as i32) > 0 && (node.0 as i32 - shift as i32) > 0 {
            nodes.push((node.0 - shift, node.1 - shift));
        }

        shift += 1;
    }

    nodes
}
