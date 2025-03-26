#![allow(unused)]
use crate::grid;
use crate::grid::{Direction, Grid, Point, Vector};

type RegionID = usize;
type Count = u32;

#[derive(Debug, Clone, PartialEq)]
pub struct RegionMap {
    grid: Grid<u8>,
    regions: Grid<Option<RegionID>>,
    no_regions: RegionID,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
struct RegionCounts {
    area: Count,
    perimeter: Count,
}

impl RegionMap {
    pub fn new(input: &str) -> RegionMap {
        let grid: Grid<u8> = input.into();
        let regions: Grid<Option<RegionID>> = Grid::new_default(grid.width, grid.height);

        let mut region_map = RegionMap {
            grid,
            regions,
            no_regions: 0,
        };
        region_map.initialise_regions();
        region_map
    }

    pub fn initialise_regions(&mut self) {
        let mut region_id = 0;

        for (point, &v) in &self.grid {
            if self.regions[point].is_none() {
                Self::set_region(
                    &self.grid,
                    &mut self.regions,
                    point,
                    region_id,
                    self.grid[point],
                );
                region_id += 1;
            }
        }
        self.no_regions = region_id;
    }

    //set the region in a cell and on all orthogonally connected cells with the same grid value, recursively
    fn set_region(
        grid: &Grid<u8>,
        regions: &mut Grid<Option<RegionID>>,
        point: Point,
        region_id: RegionID,
        cell_value: u8,
    ) {
        regions[point] = Some(region_id);

        for (neighbour, &neighbour_value) in grid.orthogonal_neighbors(point) {
            if regions[neighbour].is_none() && neighbour_value == cell_value {
                Self::set_region(grid, regions, neighbour, region_id, cell_value);
            }
        }
    }

    fn region_counts(&self) -> Vec<RegionCounts> {
        let mut region_counts = vec![
            RegionCounts {
                area: 0,
                perimeter: 0
            };
            self.no_regions
        ];

        for (point, &region_id) in &self.regions {
            if let Some(region_id) = region_id {
                region_counts[region_id].area += 1;

                for (neighbour, &neighbour_region_id) in self.regions.orthogonal_neighbors(point) {
                    if neighbour_region_id != Some(region_id) {
                        region_counts[region_id].perimeter += 1;
                    }
                }
                if point.x == 0 || point.x == self.grid.width - 1 {
                    region_counts[region_id].perimeter += 1;
                }
                if point.y == 0 || point.y == self.grid.height - 1 {
                    region_counts[region_id].perimeter += 1;
                }
            }
        }

        region_counts
    }

    pub fn price(&self) -> u32 {
        let region_counts = self.region_counts();
        let mut price = 0;

        for region in region_counts {
            price += region.area * region.perimeter;
        }

        price
    }

    fn count_sides(&self) -> Vec<RegionCounts> {
        let mut region_counts = vec![
            RegionCounts {
                area: 0,
                perimeter: 0
            };
            self.no_regions
        ];

        for y in 0..self.regions.height {
            let mut current_top_edge: Option<RegionID> = None;
            let mut current_bottom_edge: Option<RegionID> = None;
            for x in 0..self.regions.width {
                let region_id = self.regions[Point { x, y }].unwrap();
                region_counts[region_id].area += 1;
                if (y == 0 || self.regions[Point { x, y: y - 1 }] != Some(region_id)) {
                    if current_top_edge != Some(region_id) {
                        region_counts[region_id].perimeter += 1;
                        current_top_edge = Some(region_id);
                    }
                } else {
                    current_top_edge = None;
                }
                if (y == self.regions.height - 1
                    || self.regions[Point { x, y: y + 1 }] != Some(region_id))
                {
                    if current_bottom_edge != Some(region_id) {
                        region_counts[region_id].perimeter += 1;
                        current_bottom_edge = Some(region_id);
                    }
                } else {
                    current_bottom_edge = None;
                }
            }
        }

        for x in 0..self.regions.width {
            let mut current_left_edge: Option<RegionID> = None;
            let mut current_right_edge: Option<RegionID> = None;
            for y in 0..self.regions.height {
                let region_id = self.regions[Point { x, y }].unwrap();
                if (x == 0 || self.regions[Point { x: x - 1, y }] != Some(region_id)) {
                    if current_left_edge != Some(region_id) {
                        region_counts[region_id].perimeter += 1;
                        current_left_edge = Some(region_id);
                    }
                } else {
                    current_left_edge = None;
                }
                if (x == self.regions.width - 1
                    || self.regions[Point { x: x + 1, y }] != Some(region_id))
                {
                    if current_right_edge != Some(region_id) {
                        region_counts[region_id].perimeter += 1;
                        current_right_edge = Some(region_id);
                    }
                } else {
                    current_right_edge = None;
                }
            }
        }

        region_counts
    }

    pub fn price_from_sides(&self) -> u32 {
        let region_counts = self.count_sides();
        let mut price = 0;

        for region in region_counts {
            price += region.area * region.perimeter;
        }

        price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = crate::TESTINPUT;
        let region_map = RegionMap::new(input);

        assert_eq!(region_map.price_from_sides(), 1206);
    }

    #[test]
    fn test_count_sides() {
        let input = "111\n101\n111";
        let mut region_map = RegionMap::new(input);

        let region_counts = region_map.count_sides();

        let expected = vec![
            RegionCounts {
                area: 8,
                perimeter: 8,
            },
            RegionCounts {
                area: 1,
                perimeter: 4,
            },
        ];

        assert_eq!(region_counts, expected);
    }

    #[test]
    fn test_initialise_regions() {
        let input = "111\n101\n111";
        let mut region_map = RegionMap::new(input);

        let expected = vec![
            vec![Some(0), Some(0), Some(0)],
            vec![Some(0), Some(1), Some(0)],
            vec![Some(0), Some(0), Some(0)],
        ];

        assert_eq!(region_map.regions, expected.into());
        assert_eq!(region_map.no_regions, 2);
    }

    #[test]
    fn test_region_counts() {
        let input = "111\n101\n111";
        let mut region_map = RegionMap::new(input);

        let region_counts = region_map.region_counts();

        let expected = vec![
            RegionCounts {
                area: 8,
                perimeter: 16,
            },
            RegionCounts {
                area: 1,
                perimeter: 4,
            },
        ];

        assert_eq!(region_counts, expected);
    }

    #[test]
    fn test_price() {
        let input = "111\n101\n111";
        let mut region_map = RegionMap::new(input);

        let price = region_map.price();

        assert_eq!(price, 8 * 16 + 4);
    }
}
