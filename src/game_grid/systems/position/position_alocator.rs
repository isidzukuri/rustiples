use rand::prelude::SliceRandom;
#[derive(Copy, Clone, Debug)]
pub struct PositionAllocation {
    pub from_x_cell: u32,
    pub from_y_cell: u32,
    pub to_x_cell: u32,
    pub to_y_cell: u32,
}
pub struct PositionAllocator {
    pub width_cells: u32,
    pub height_cells: u32,
    pub reserved_cells: Vec<(u32, u32)>,
}

impl PositionAllocator {
    pub fn reserve(&mut self, col_index: u32, row_index: u32) {
        let node = (col_index, row_index);
        if self.reserved_cells.contains(&node) {
            panic!("Position already reserved");
        }

        self.reserved_cells.push(node);
    }

    pub fn allocate(&mut self, width_cells: u32, height_cells: u32) -> Option<PositionAllocation> {
        let mut col_index: u32 = 0;
        let mut row_index: u32 = 0;
        let mut variants = vec![];

        loop {
            if row_index == self.height_cells && col_index == 0 {
                break;
            }
            if (col_index + width_cells) < self.width_cells
                && (row_index + height_cells) < self.height_cells
            {
                let mut allocated = true;

                for cur_col_index in col_index..(col_index + width_cells) {
                    for cur_row_index in row_index..(row_index + height_cells) {
                        if self
                            .reserved_cells
                            .contains(&(cur_col_index, cur_row_index))
                        {
                            allocated = false;
                            break;
                        }
                    }
                    if allocated == false {
                        break;
                    }
                }

                if allocated {
                    variants.push(PositionAllocation {
                        from_x_cell: col_index,
                        from_y_cell: row_index,
                        to_x_cell: col_index + width_cells - 1,
                        to_y_cell: row_index + height_cells - 1,
                    });
                }
            }

            col_index += 1;
            if col_index == self.width_cells {
                col_index = 0;
                row_index += 1;
            };
        }

        match variants.choose(&mut rand::thread_rng()) {
            None => None,
            Some(allocation) => {
                for cur_col_index in allocation.from_x_cell..allocation.to_x_cell {
                    for cur_row_index in allocation.from_y_cell..allocation.to_y_cell {
                        self.reserved_cells.push((cur_col_index, cur_row_index));
                    }
                }

                Some(*allocation)
            }
        }
    }
}
