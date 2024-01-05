use super::position_alocator::*;

#[derive(Debug, Clone)]
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

    pub fn alocate_new_position(
        sprite_width: &f32,
        sprite_height: &f32,
        window_width_in_cells: &u32,
        window_height_in_cells: &u32,
        cell_size: &f32,
        margin: &(u32, u32, u32, u32), // clockwise from 12
        position_allocator: &mut PositionAllocator,
    ) -> Self {
        let width_cells = (sprite_width / cell_size).ceil() as u32;
        let height_cells = (sprite_height / cell_size).ceil() as u32;

        let width_cells_with_box = width_cells + margin.1 + margin.3;
        let height_cells_with_box = height_cells + margin.0 + margin.2;

        match position_allocator.allocate(width_cells_with_box, height_cells_with_box) {
            None => {
                panic!("not possible to allocate space in world for the object")
            }
            Some(allocation) => Self {
                width_px: *sprite_width,
                height_px: *sprite_height,
                width_cells: width_cells,
                height_cells: height_cells,

                from_x_cell: allocation.from_x_cell + margin.3,
                from_y_cell: allocation.from_y_cell + margin.2,
                to_x_cell: allocation.to_x_cell - margin.1,
                to_y_cell: allocation.to_y_cell - margin.0,
                margin: *margin,
            },
        }
    }

    pub fn alocate_at(
        position_x_cell: &u32,
        position_y_cell: &u32,
        sprite_width: &f32,
        sprite_height: &f32,
        cell_size: &f32,
        margin: &(u32, u32, u32, u32), // clockwise from 12
        position_allocator: &mut PositionAllocator,
    ) -> Self {
        let width_in_cells = (sprite_width / cell_size).ceil() as u32;
        let height_in_cells = (sprite_height / cell_size).ceil() as u32;

        position_allocator.reserve(*position_x_cell, *position_y_cell);

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
