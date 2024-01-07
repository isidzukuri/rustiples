use rand::prelude::SliceRandom;
#[derive(Copy, Clone, Debug)]
pub struct PositionAllocation {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}
pub struct PositionAllocator {
    pub width: u32,
    pub height: u32,
    pub reserved_cells: Vec<(u32, u32)>,
}

impl PositionAllocator {
    pub fn free(&mut self, col_index: u32, row_index: u32) {
        // remove from self.reserved_cells
    }

    pub fn reserve(&mut self, col_index: u32, row_index: u32) {
        let node = (col_index, row_index);
        if self.reserved_cells.contains(&node) {
            panic!("Position already reserved");
        }

        self.reserved_cells.push(node);
    }

    pub fn allocate(&mut self, width: u32, height: u32) -> Option<PositionAllocation> {
        let mut col_index: u32 = 0;
        let mut row_index: u32 = 0;
        let mut variants = vec![];

        loop {
            if row_index == self.height && col_index == 0 {
                break;
            }
            if (col_index + width) < self.width && (row_index + height) < self.height {
                let mut allocated = true;

                for cur_col_index in col_index..(col_index + width) {
                    for cur_row_index in row_index..(row_index + height) {
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
                        x1: col_index,
                        y1: row_index,
                        x2: col_index + width - 1,
                        y2: row_index + height - 1,
                    });
                }
            }

            col_index += 1;
            if col_index == self.width {
                col_index = 0;
                row_index += 1;
            };
        }

        match variants.choose(&mut rand::thread_rng()) {
            None => None,
            Some(allocation) => {
                for cur_x in allocation.x1..allocation.x2 {
                    for cur_y in allocation.y1..allocation.y2 {
                        self.reserved_cells.push((cur_x, cur_y));
                    }
                }

                Some(*allocation)
            }
        }
    }
}
