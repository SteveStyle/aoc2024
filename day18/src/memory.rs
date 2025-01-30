type Generation = usize;

use stephen_morris_utils::get_numbers;

use crate::{
    constants::{HEIGHT, WIDTH},
    grid::{Direction, Grid, Point, Vector},
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum PathStatus {
    #[default]
    NotKnown,
    Unreachable,
    ReachableIn(Generation),
    Reachable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Memory {
    grid_corrupt_from: Grid<Generation>,
    byte_list: Vec<Point>,
    start_point: Point,
    end_point: Point,
}

impl Memory {
    pub fn new(input: &str) -> Self {
        let mut grid_corrupt_from: Grid<Generation> = Grid::new(WIDTH, HEIGHT, Generation::MAX);
        let mut byte_list = Vec::with_capacity(3450);
        for (generation, line) in input.lines().enumerate() {
            let mut numbers = get_numbers(line);
            if numbers.len() >= 2 {
                let point = Point {
                    x: numbers[0],
                    y: numbers[1],
                };
                byte_list.push(point);
                grid_corrupt_from[point] = generation as Generation;
            }
        }
        let start_point = Point::new(0, 0);
        let end_point = Point::new(WIDTH - 1, HEIGHT - 1);
        Memory {
            grid_corrupt_from,
            byte_list,
            start_point,
            end_point,
        }
    }

    pub fn min_path_period(&self, period: Generation) -> Generation {
        fn set_reachable_in(grid: &mut Grid<PathStatus>, point: Point, generation: Generation) {
            if grid[point] == PathStatus::Unreachable {
                return;
            }
            if let PathStatus::ReachableIn(old_generation) = grid[point] {
                if old_generation <= generation {
                    return;
                }
            }
            grid[point] = PathStatus::ReachableIn(generation);
            let v: Vec<Point> = grid.orthogonal_neighbors(point).map(|(p, _)| p).collect();
            for next_point in v {
                set_reachable_in(grid, next_point, generation + 1);
            }
        }
        let mut grid = Grid::new_default(WIDTH, HEIGHT);
        for (point, &corrupt_from) in &self.grid_corrupt_from {
            if corrupt_from < period {
                grid[point] = PathStatus::Unreachable;
            }
        }
        set_reachable_in(&mut grid, Point::new(0, 0), 0);
        if let PathStatus::ReachableIn(generation) = grid[Point::new(WIDTH - 1, HEIGHT - 1)] {
            generation
        } else {
            unreachable!()
        }
    }

    fn is_connected_after(&self, period: Generation) -> bool {
        fn set_reachable(grid: &mut Grid<PathStatus>, point: Point, end_point: Point) -> bool {
            if grid[point] != PathStatus::NotKnown {
                return false;
            }
            grid[point] = PathStatus::Reachable;
            let v: Vec<Point> = grid.orthogonal_neighbors(point).map(|(p, _)| p).collect();
            for next_point in v {
                if next_point == end_point || set_reachable(grid, next_point, end_point) {
                    return true;
                }
            }
            false
        }

        let mut grid = Grid::new_default(WIDTH, HEIGHT);
        for (point, &corrupt_from) in &self.grid_corrupt_from {
            if corrupt_from < period {
                grid[point] = PathStatus::Unreachable;
            }
        }
        set_reachable(&mut grid, self.start_point, self.end_point)
    }

    pub fn find_disconnection(&self) -> Point {
        let mut start_range = 0;
        let mut end_range = self.byte_list.len();
        while start_range != end_range {
            let period = (start_range + end_range + 1) >> 1;
            if self.is_connected_after(period) {
                start_range = period;
            } else {
                end_range = period - 1;
            }
        }
        self.byte_list[start_range]
    }
}
