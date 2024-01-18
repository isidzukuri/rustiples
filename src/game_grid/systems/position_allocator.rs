use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PositionAllocation {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

#[derive(Serialize, Deserialize)]
pub struct PositionAllocator {
    pub width: u32,
    pub height: u32,
    pub reserved_cells: Vec<(u32, u32)>,
}

impl PositionAllocator {
    pub fn allocate_coords(
        &mut self,
        coords: (u32, u32),
        width: u32,
        height: u32,
    ) -> Option<PositionAllocation> {
        if self.reserved_cells.contains(&coords) {
            panic!("Position already reserved");
        }
        // TODO: check in range of width/height positions are available
        // TODO: validate widt, height

        self.reserved_cells.push(coords);

        Some(PositionAllocation {
            x1: coords.0,
            y1: coords.1,
            x2: coords.0 + width,
            y2: coords.1 + height,
        })
    }

    pub fn allocate(
        &mut self,
        width: u32,
        height: u32,
        margin: (u32, u32, u32, u32),
    ) -> Option<PositionAllocation> {
        let mut col_index: u32 = 0;
        let mut row_index: u32 = 0;
        let mut variants = vec![];

        let width_with_box = width + margin.1 + margin.3;
        let height_with_box = height + margin.0 + margin.2;

        loop {
            if row_index == self.height && col_index == 0 {
                break;
            }
            if (col_index + width_with_box) < self.width
                && (row_index + height_with_box) < self.height
            {
                let mut allocated = true;

                for cur_col_index in col_index..(col_index + width_with_box) {
                    for cur_row_index in row_index..(row_index + height_with_box) {
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
                        x1: col_index + margin.3,
                        y1: row_index + margin.2,
                        x2: col_index + width + margin.3,
                        y2: row_index + height + margin.2,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_coords() {
        let mut position_allocator = PositionAllocator {
            width: 10,
            height: 10,
            reserved_cells: vec![],
        };

        let coords = (1u32, 1u32);
        let width = 0u32;
        let height = 0u32;

        let allocation = position_allocator
            .allocate_coords(coords, width, height)
            .unwrap();
        assert_eq!(allocation.x1, 1);
        assert_eq!(allocation.y1, 1);
        assert_eq!(allocation.x2, 1);
        assert_eq!(allocation.y2, 1);

        let coords = (2u32, 4u32);
        let width = 3u32;
        let height = 5u32;

        let allocation = position_allocator
            .allocate_coords(coords, width, height)
            .unwrap();
        assert_eq!(allocation.x1, 2);
        assert_eq!(allocation.y1, 4);
        assert_eq!(allocation.x2, 5);
        assert_eq!(allocation.y2, 9);
    }

    #[test]
    #[should_panic(expected = "Position already reserved")]
    fn test_allocate_coords_failure() {
        let mut position_allocator = PositionAllocator {
            width: 10,
            height: 10,
            reserved_cells: vec![],
        };

        let coords = (1u32, 1u32);
        let width = 0u32;
        let height = 0u32;

        let _allocation = position_allocator.allocate_coords(coords, width, height);
        let _allocation = position_allocator.allocate_coords(coords, width, height);
    }

    #[test]
    fn test_allocate() {
        let mut position_allocator = PositionAllocator {
            width: 10,
            height: 10,
            reserved_cells: vec![],
        };

        let width = 10u32;
        let height = 10u32;
        let margin = (0u32, 0u32, 0u32, 0u32);

        let allocation = position_allocator.allocate(width, height, margin);
        assert_eq!(allocation.is_some(), false);

        let width = 1u32;
        let height = 1u32;
        let margin = (5u32, 5u32, 5u32, 5u32);

        let allocation = position_allocator.allocate(width, height, margin);
        assert_eq!(allocation.is_some(), false);

        let width = 1u32;
        let height = 1u32;
        let margin = (3u32, 1u32, 0u32, 0u32);

        let allocation = position_allocator.allocate(width, height, margin);
        assert_eq!(allocation.is_some(), true);

        let width = 2u32;
        let height = 2u32;
        let margin = (1u32, 0u32, 0u32, 1u32);

        let allocation = position_allocator.allocate(width, height, margin);
        assert_eq!(allocation.is_some(), true);

        let width = 1u32;
        let height = 1u32;
        let margin = (0u32, 0u32, 0u32, 0u32);

        let allocation = position_allocator.allocate(width, height, margin);
        assert_eq!(allocation.is_some(), true);
    }

    #[test]
    fn test_release_coords() {
        let mut position_allocator = PositionAllocator {
            width: 1,
            height: 1,
            reserved_cells: vec![],
        };

        let coords = (1u32, 1u32);
        let width = 0u32;
        let height = 0u32;
        let margin = (0u32, 0u32, 0u32, 0u32);

        let _ = position_allocator.allocate_coords(coords, width, height);
        let allocation = position_allocator.allocate(width, height, margin);
        assert_eq!(allocation.is_some(), false);

        position_allocator.release_coords(vec![coords]);

        let allocation = position_allocator.allocate(width, height, margin);
        assert_eq!(allocation.is_some(), true);
    }
}
