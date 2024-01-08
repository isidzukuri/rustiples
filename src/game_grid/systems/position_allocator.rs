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
    pub fn allocate_coords(&mut self, coords: (u32, u32)) -> Option<PositionAllocation> {
        if self.reserved_cells.contains(&coords) {
            panic!("Position already reserved");
        }

        self.reserved_cells.push(coords);

        Some(PositionAllocation {
            x1: coords.0,
            y1: coords.1,
            x2: coords.0 + 1,
            y2: coords.1 + 1,
        })
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
                        x2: col_index + width,
                        y2: row_index + height,
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

    pub fn release_coords(&mut self, coords_list: Vec<(u32, u32)>) {
        for coords in coords_list {
            self.reserved_cells.retain(|&cell| cell != coords);
        }
    }
}
