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
}
