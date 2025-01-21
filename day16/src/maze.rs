use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    slice::SliceIndex,
    thread::current,
};

use crate::grid::{Direction, Grid, Point, Vector, DOWN, LEFT, RIGHT, UP};

type Count = u32;

pub fn part1(input: &str) -> Count {
    Maze::new(input).find_best_score()
}
pub fn part2(input: &str) -> Count {
    Maze::new(input).cells_on_optimal_path()
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum FindPathStatus {
    #[default]
    Ready,
    FindPathNeeded,
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct CellDirectionPath {
    status: FindPathStatus,
    best_from_start: Option<Count>,
    best_to_end: Option<Count>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PathError {
    LoopDetected,
    TotalExceeded,
    NoValidPath,
}
impl CellDirectionPath {
    fn touch(&mut self, mut new_from_start: Count, best_total: Count) -> Result<(), PathError> {
        if self.on_loop() {
            unreachable!() // should have been caught at the start of find path
                           // return Err(PathError::LoopDetected);
        }
        if new_from_start >= best_total {
            return Err(PathError::TotalExceeded);
        }
        if let Some(current_from_start) = self.best_from_start {
            if current_from_start < new_from_start {
                new_from_start = current_from_start;
            } else {
                self.best_from_start = Some(new_from_start);
            }
        } else {
            self.best_from_start = Some(new_from_start);
        }
        self.status = FindPathStatus::FindPathNeeded;
        Ok(())
    }
    fn on_loop(&self) -> bool {
        match self.status {
            FindPathStatus::Ready => false,
            FindPathStatus::FindPathNeeded => true,
        }
    }
    fn try_get_total(&self) -> Option<Count> {
        Some(self.best_from_start? + self.best_to_end?)
    }
    fn path_found(&mut self, new_to_end: Count) {
        if self
            .best_to_end
            .is_none_or(|best_to_end| best_to_end > new_to_end)
        {
            self.best_to_end = Some(new_to_end);
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct CellDirectionPaths {
    paths: [CellDirectionPath; 4],
}

impl Deref for CellDirectionPaths {
    type Target = [CellDirectionPath];
    fn deref(&self) -> &Self::Target {
        &self.paths
    }
}

impl DerefMut for CellDirectionPaths {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.paths
    }
}

impl CellDirectionPaths {
    // update the start_from values when we first consider a cell
    fn on_loop(&mut self) -> bool {
        self.paths.iter().any(|&cdp| cdp.on_loop())
    }
    fn best_total(&self) -> Option<Count> {
        self.paths
            .iter()
            .filter_map(CellDirectionPath::try_get_total)
            .min()
    }
}

#[derive(Debug)]
pub struct Maze {
    maze: Grid<u8>,
    counts: Grid<CellDirectionPaths>,
    best_path: Grid<bool>,
    start: Point,
    end: Point,
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let maze: Grid<u8> = input.into();
        let counts = Grid::new_default(maze.width, maze.height);
        let best_path = Grid::new(maze.width, maze.height, false);
        let start = maze.find(b'S').unwrap();
        let end = maze.find(b'E').unwrap();

        Maze {
            maze,
            counts,
            best_path,
            start,
            end,
        }
    }

    pub fn find_best_score(&mut self) -> Count {
        fn find_path(
            maze: &Grid<u8>,
            counts: &mut Grid<CellDirectionPaths>,
            current_cell: Point,
            current_direction: Direction,
            from_start: Count, // the score for reaching the this cell, prior to turning
            mut best_total: Count,
        ) -> Result<Count, PathError> {
            // find_path() starts from a particular cell, pointing in a particular direction, along with the score required to reach this position.
            // It considers each of the three possible turns (left, right, straight on) followed by a move forward, and then calls itself recursively.
            // It returns the shortest score it finds to reach the End square from that starting position.
            //
            // Each cell/position stores the lowest score for reaching that cell/position from the startion position, and the lowest score for reaching the end from that position.
            //
            // best_total is the best total score found for the whole path from start to end.  It is only passed around via this function and is not stored in the cells.

            if from_start > best_total {
                return Err(PathError::TotalExceeded);
            }
            if maze[current_cell] == b'E' {
                return Ok(0);
            }
            if counts[current_cell].on_loop() {
                return Err(PathError::LoopDetected);
            }
            // Add the score for turning
            // So new_from_start represents the total score for reaching this position in this cell and turning in one of the three directions.
            // It does not include the score for moving into the next cell.
            let directions = [
                (current_direction, 0),
                (current_direction.left(), 1000),
                (current_direction.right(), 1000),
            ];
            for (new_direction, turn) in directions {
                if let Some(next_cell) = maze.add_vector(current_cell, Vector::from(new_direction))
                {
                    match maze[next_cell] {
                        b'.' | b'E' => {
                            // toucn() updates the best from_start and marks the cell/direction for a recursive call to find_path
                            counts[current_cell][new_direction as usize]
                                .touch(from_start + turn, best_total);
                        }
                        b'S' => return Err(PathError::LoopDetected),
                        b'#' => {}
                        _ => panic!("find_best_path: unkown value found in grid"),
                    }
                }
            }

            let mut best_to_end = Count::MAX;
            let mut path_found = false;
            for (new_direction, turn) in directions {
                if let Some(next_cell) = maze.add_vector(current_cell, Vector::from(new_direction))
                {
                    if counts[current_cell][new_direction as usize].status
                        == FindPathStatus::FindPathNeeded
                    {
                        if let Ok(new_to_end) = find_path(
                            maze,
                            counts,
                            next_cell,
                            new_direction,
                            from_start + turn + 1, // add the score for moving to the next cell, ready to turn
                            best_total,
                        ) {
                            // update the best to_end for the cell/direction
                            // update the best_total
                            counts[current_cell][new_direction as usize].path_found(new_to_end + 1); // allow for the turn and step to the next cell
                            best_total = best_total.min(from_start + turn + 1 + new_to_end);
                            best_to_end = best_to_end.min(turn + 1 + new_to_end);
                            path_found = true;
                        }
                    }
                    counts[current_cell][new_direction as usize].status = FindPathStatus::Ready;
                }
            }

            if path_found {
                Ok(best_to_end)
            } else {
                Err(PathError::NoValidPath)
            }
        }
        // self.counts[self.start][RIGHT].set_from_start(0);
        if let Ok(best_score) = find_path(
            &self.maze,
            &mut self.counts,
            self.start,
            Direction::Right,
            0,
            Count::MAX,
        ) {
            best_score
        } else {
            panic!("could not solve maze")
        }
    }

    pub fn cells_on_optimal_path(&mut self) -> Count {
        let mut optimal_cells = self.maze.clone();
        let best_score = self.find_best_score();
        let mut count = 2; // count S and E up front
        for (point, &value) in &self.maze {
            if value == b'.' {
                if let Some(total) = self.counts[point].best_total() {
                    if total == best_score {
                        optimal_cells[point] = b'O';
                        count += 1;
                    }
                }
            }
        }
        optimal_cells.print();
        count
    }
}
