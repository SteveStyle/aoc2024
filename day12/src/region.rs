#[allow(unused)]
use crate::grid;
use crate::grid::Grid;

pub struct RegionMap {
    grid: Grid<u8>,
    regions: Grid<u8>,
}

impl RegionMap {
    pub fn new(input: &str) -> RegionMap {
        let grid: Grid<u8> = input.into();
        let mut regions: Grid<u8> = Grid::new_default(grid.width, grid.height);

        let mut region_id = 0;

        for (point, &v) in &grid {
            if regions[point] == 0 {
                regions[point] = region_id;
                for (neighbor, &neigbour_value) in grid.orthogonal_neighbors(point) {
                    if regions[neighbor] == 0 && neigbour_value == v {
                        regions[neighbor] = region_id;
                    }
                }
                region_id += 1;
            }
        }

        RegionMap { grid, regions }
    }
}
