use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Debug, Clone)]
pub struct WorldPosition {
    pub width_px: f32,
    pub height_px: f32,
    pub width_cells: u32,
    pub height_cells: u32,
    pub from_x_cell: u32,
    pub from_y_cell: u32,
    pub to_x_cell: u32,
    pub to_y_cell: u32,
    pub margin: (u32, u32, u32, u32),
}

impl WorldPosition {
    pub fn is_owned_cell(&self, x: &u32, y: &u32) -> bool {
        if self.from_x_cell <= *x
            && self.to_x_cell >= *x
            && self.from_y_cell <= *y
            && self.to_y_cell >= *y
        {
            true
        } else {
            false
        }
    }

    pub fn intersects_with(&self, other: &Self) -> bool {
        if (((self.from_x_cell_with_margin() <= other.to_x_cell_with_margin()
            && self.to_x_cell_with_margin() >= other.from_x_cell_with_margin())
            || (self.from_x_cell_with_margin() >= other.to_x_cell_with_margin()
                && self.to_x_cell_with_margin() <= other.from_x_cell_with_margin()))
            && (self.from_y_cell_with_margin() >= other.to_y_cell_with_margin()
                && self.to_y_cell_with_margin() <= other.from_y_cell_with_margin())
            || self.from_y_cell_with_margin() <= other.to_y_cell_with_margin()
                && self.to_y_cell_with_margin() >= other.from_y_cell_with_margin())
        {
            true
        } else {
            false
        }
    }

    fn from_x_cell_with_margin(&self) -> u32 {
        self.from_x_cell - self.margin.3
    }

    fn to_x_cell_with_margin(&self) -> u32 {
        self.to_x_cell + self.margin.1
    }

    fn from_y_cell_with_margin(&self) -> u32 {
        self.from_y_cell - self.margin.2
    }

    fn to_y_cell_with_margin(&self) -> u32 {
        self.to_y_cell + self.margin.0
    }

    pub fn alocate_new_position(
        sprite_width: &f32,
        sprite_height: &f32,
        window_width_in_cells: &u32,
        window_height_in_cells: &u32,
        cell_size: &f32,
        margin: &(u32, u32, u32, u32), // clockwise from 12
    ) -> Self {
        let width_in_cells = (sprite_width / cell_size).ceil() as u32;
        let height_in_cells = (sprite_height / cell_size).ceil() as u32;

        let max_x_cell = window_width_in_cells - width_in_cells - margin.1;
        let max_y_cell = window_height_in_cells - height_in_cells - margin.0;

        let position_x_cell: u32 = rand::thread_rng().gen_range(margin.3..max_x_cell);
        let position_y_cell: u32 = rand::thread_rng().gen_range(margin.2..max_y_cell);

        Self {
            width_px: *sprite_width,
            height_px: *sprite_height,
            width_cells: width_in_cells,
            height_cells: height_in_cells,
            from_x_cell: position_x_cell,
            from_y_cell: position_y_cell,
            to_x_cell: position_x_cell + width_in_cells - 1,
            to_y_cell: position_y_cell + height_in_cells - 1,
            margin: *margin,
        }
    }

    pub fn alocate_at(
        position_x_cell: &u32,
        position_y_cell: &u32,
        sprite_width: &f32,
        sprite_height: &f32,
        cell_size: &f32,
        margin: &(u32, u32, u32, u32), // clockwise from 12
    ) -> Self {
        let width_in_cells = (sprite_width / cell_size).ceil() as u32;
        let height_in_cells = (sprite_height / cell_size).ceil() as u32;

        Self {
            width_px: *sprite_width,
            height_px: *sprite_height,
            width_cells: width_in_cells,
            height_cells: height_in_cells,
            from_x_cell: *position_x_cell,
            from_y_cell: *position_y_cell,
            to_x_cell: position_x_cell + width_in_cells - 1,
            to_y_cell: position_y_cell + height_in_cells - 1,
            margin: *margin,
        }
    }
}


pub trait WorldPositionParams {
    // (sprite_width: &f32, sprite_height: &f32, margin: &(u32, u32, u32, u32))
    fn world_position_params() -> (f32, f32, (u32, u32, u32, u32)); 
}
