use std::collections::HashSet;

use stephen_morris_utils::pos::{Direction, };

type Grid = crate::grid::Grid<u8>;
type Position = stephen_morris_utils::pos::Position<usize>;

pub fn parse_input(input: &str) -> Grid {
    let grid = Grid::from(input);
    grid
}

pub fn extract_guard(grid: &mut Grid) -> Option<Guard> {
    let mut ret = None;
    for (row, col, c) in &*grid {
        match c {
            b'^' => {grid.set(row, col, b'.'); ret = Some(Guard::new(row, col, Direction::Up) ); break;},
            b'v' => {grid.set(row, col, b'.'); ret = Some(Guard::new(row, col, Direction::Down)); break;},
            b'<' => {grid.set(row, col, b'.'); ret = Some(Guard::new(row, col, Direction::Left)); break;},
            b'>' => {grid.set(row, col, b'.'); ret = Some(Guard::new(row, col, Direction::Right)); break;},
            _ => {}

        }
    }
    ret
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Guard {
    pos: Position,
    direction: Direction,
}  

impl Guard {
    pub fn new_pos(pos: Position, direction: Direction) -> Self {
        Self {pos, direction}
    }
    pub fn new(row: usize, col: usize, direction: Direction) -> Self {
        Self {pos: Position::new(col, row), direction}
    }
    pub fn move_once(&mut self, grid: &Grid) -> bool {
        if grid.test_bounds( self.pos, self.direction) {         
            let new_pos = self.pos + self.direction;
            if *grid.get_pos(new_pos) == b'#' {
                self.direction = self.direction.turn_right();
            } else {
                self.pos = new_pos;
            }
            true
        } else {
            false
        }
    
    }
    pub fn count_guard_positions(&mut self, grid: &Grid) -> usize {
        let mut guard_positions = Grid::new_default(grid.width, grid.height);
        guard_positions.set_pos(self.pos, 1);
        while self.move_once(grid) {
            guard_positions.set_pos(self.pos, 1);
        }
        guard_positions.into_iter().map(|(_,_,v)| v as usize).sum()
    }

    pub fn count_blockers(&self, grid: &Grid) -> usize {
        let mut guard_positions = Grid::new_default(grid.width, grid.height);

        let mut guard_copy = *self;
        let mut grid_copy = grid.clone();
        
        while guard_copy.move_once(grid) {
            guard_positions.set_pos(guard_copy.pos, 1);
        }

        let position_count: usize = (&guard_positions).into_iter().map(|(_,_,v)| *v as usize).sum();
        println!("{:?}", position_count);

        let mut possible_blockers = Vec::new();

        for (row, col, c) in &guard_positions {
            if *c == 1 {
                grid_copy.set(row, col, b'#');
                guard_copy = *self;
                let mut hs = HashSet::new();
                while guard_copy.move_once(&grid_copy) {
                    if !hs.insert(guard_copy.clone()) {
                        possible_blockers.push((row, col));
                        break;
                    }
                }
                grid_copy.set(row, col, b'.');
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
        let mut guard = extract_guard(&mut grid).unwrap();
        assert_eq!(guard.count_blockers(&grid), 6);
    }
}
