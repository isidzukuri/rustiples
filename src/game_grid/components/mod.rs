pub mod grid;
// pub mod grid_node;
// pub mod grid_node;
// pub mod axe;
// pub mod castle;
// pub mod graph_node;
// pub mod hero;
// pub mod mineral;
// pub mod mountain;
// pub mod tree;


use bevy::prelude::*;
use bevy::window::PrimaryWindow;
// use rand::Rng;
use crate::game_grid::grid::*;



// generate_grid


pub const GRID_CELL_WIDTH: f32 = 50.0 as f32;
pub const HALF_GRID_CELL_WIDTH: f32 = 25.0 as f32;

pub fn generate_grid(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let width = (window.width() / GRID_CELL_WIDTH) as u32;
    let height = (window.height() / GRID_CELL_WIDTH) as u32;

    let (grid, nodes) = Grid::new(width, height);


    let half_size = GRID_CELL_WIDTH / 2.0;
    for node in nodes {
        let window_x = GRID_CELL_WIDTH * node.x as f32 + half_size;
        let window_y = GRID_CELL_WIDTH * node.y as f32 + half_size;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(GRID_CELL_WIDTH, GRID_CELL_WIDTH)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(window_x, window_y, -1.)),
                ..default()
            },
            node,
        ));
    }
    commands.insert_resource(grid);

}