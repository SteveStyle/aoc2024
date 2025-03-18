use std::collections::HashSet;

use stephen_morris_utils::grid::{Direction, Point};

type Grid = crate::grid::Grid<u8>;

pub fn parse_input(input: &str) -> Grid {
    Grid::from(input)
}

pub fn extract_guard(grid: &mut Grid) -> Option<Guard> {
    let mut ret = None;
    for (point, cell_value) in &*grid {
        match cell_value {
            b'^' => {
                grid.set(point, b'.');
                ret = Some(Guard::new(point, Direction::North));
                break;
            }
            b'v' => {
                grid.set(point, b'.');
                ret = Some(Guard::new(point, Direction::South));
                break;
            }
            b'<' => {
                grid.set(point, b'.');
                ret = Some(Guard::new(point, Direction::West));
                break;
            }
            b'>' => {
                grid.set(point, b'.');
                ret = Some(Guard::new(point, Direction::East));
                break;
            }
            _ => {}
        }
    }
    ret
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Guard {
    pos: Point,
    direction: Direction,
}

impl Guard {
    pub fn new(pos: Point, direction: Direction) -> Self {
        Self { pos, direction }
    }

    pub fn move_once(&mut self, grid: &Grid) -> bool {
        match grid.add_direction(self.pos, self.direction) {
            Some(next_pos) => {
                if grid[next_pos] == b'#' {
                    self.direction = self.direction.right()
                } else {
                    self.pos = next_pos
                };
                true
            }
            None => false,
        }
    }
    pub fn count_guard_positions(&mut self, grid: &Grid) -> usize {
        let mut guard_positions = Grid::new_default(grid.width, grid.height);
        guard_positions[self.pos] = 1;
        while self.move_once(grid) {
            guard_positions[self.pos] = 1;
        }
        guard_positions.into_iter().map(|(_, v)| v as usize).sum()
    }

    pub fn count_blockers(&self, grid: &Grid) -> usize {
        let mut guard_positions = Grid::new_default(grid.width, grid.height);

        let mut guard_copy = *self;
        let mut grid_copy = grid.clone();

        while guard_copy.move_once(grid) {
            guard_positions[guard_copy.pos] = 1;
        }

        let position_count: usize = (&guard_positions)
            .into_iter()
            .map(|(_, v)| *v as usize)
            .sum();
        println!("{:?}", position_count);

        let mut possible_blockers = Vec::new();

        for (point, point_value) in &guard_positions {
            if *point_value == 1 {
                grid_copy[point] = b'#';
                guard_copy = *self;
                let mut hs = HashSet::new();
                while guard_copy.move_once(&grid_copy) {
                    if !hs.insert(guard_copy) {
                        possible_blockers.push(point);
                        break;
                    }
                }
                grid_copy.set(point, b'.');
            }
        }

        possible_blockers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all() {
        let mut grid = parse_input(crate::TESTINPUT);
        let mut guard = extract_guard(&mut grid).unwrap();
        assert_eq!(guard.count_guard_positions(&grid), 41);
    }

    #[test]
    fn test_blockers() {
        let mut grid = parse_input(crate::TESTINPUT);
        let guard = extract_guard(&mut grid).unwrap();
        assert_eq!(guard.count_blockers(&grid), 6);
    }
}
