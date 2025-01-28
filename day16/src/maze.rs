use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    slice::SliceIndex,
    thread::current,
};

use crate::grid::{Direction, Grid, Point, Vector, EAST, NORTH, SOUTH, WEST};

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
    EqualPath(Option<Count>),
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
        match self.best_from_start {
            Some(current_best_from_start) => match current_best_from_start.cmp(&new_from_start) {
                std::cmp::Ordering::Less => {
                    self.status = FindPathStatus::Ready;
                }
                std::cmp::Ordering::Equal => {
                    self.status = FindPathStatus::EqualPath(self.best_to_end);
                }
                std::cmp::Ordering::Greater => {
                    self.best_from_start = Some(new_from_start);
                    self.status = FindPathStatus::FindPathNeeded;
                }
            },
            None => {
                self.best_from_start = Some(new_from_start);
                self.status = FindPathStatus::FindPathNeeded;
            }
        }
        Ok(())
    }
    fn on_loop(&self) -> bool {
        self.status != FindPathStatus::Ready
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

impl Display for CellDirectionPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cdp) in self.paths.iter().enumerate() {
            writeln!(
                f,
                "{:} {:6} {:6}",
                char::from(Direction::from(i)),
                match cdp.best_from_start {
                    Some(count) => count.to_string(),
                    None => "None".to_string(),
                },
                match cdp.best_to_end {
                    Some(count) => count.to_string(),
                    None => "None".to_string(),
                }
            )?;
        }
        Ok(())
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
    fn print(&self) {
        for (i, cdp) in self.paths.iter().enumerate() {
            println!(
                "{:?} {:?} {:?} {:?}",
                char::from(Direction::from(i)),
                cdp.status,
                cdp.best_from_start,
                cdp.best_to_end
            );
        }
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
            from_start: Count,
            mut best_total: Count,
        ) -> Result<Count, PathError> {
            if maze[current_cell] == b'E' {
                return Ok(0);
            }
            if from_start >= best_total {
                return Err(PathError::TotalExceeded);
            }
            if counts[current_cell].on_loop() {
                return Err(PathError::LoopDetected);
            }
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
                    match counts[current_cell][new_direction as usize].status {
                        FindPathStatus::FindPathNeeded | FindPathStatus::EqualPath(None) => {
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
                                counts[current_cell][new_direction as usize]
                                    .path_found(new_to_end + 1); // allow for the turn and step to the next cell
                                best_total = best_total.min(from_start + turn + 1 + new_to_end);
                                best_to_end = best_to_end.min(turn + 1 + new_to_end);
                                path_found = true;
                            }
                            counts[current_cell][new_direction as usize].status =
                                FindPathStatus::Ready;
                        }
                        // we can assume that if the current path is optimal then any previous path with the same from_start to this cell/direction is also optimal.
                        // Pf, suppose route A and route B pass through the same cell/direction with the same score (the same from_start) and that A is optimal but B is not.
                        // It must be that the remaining path for A passes through the previous path for B, since it is not available to B.  But then B could be improved and
                        // B was not optimal.  So the assumption is correct.
                        FindPathStatus::EqualPath(Some(new_to_end)) => {
                            // best_total = best_total.min(from_start + turn + 1 + new_to_end);
                            best_to_end = best_to_end.min(turn + new_to_end);
                            path_found = true;
                            counts[current_cell][new_direction as usize].status =
                                FindPathStatus::Ready;
                        }
                        FindPathStatus::Ready => {}
                    }
                }
            }
            if path_found {
                Ok(best_to_end)
            } else {
                Err(PathError::NoValidPath)
            }
        }
        find_path(
            &self.maze,
            &mut self.counts,
            self.start,
            crate::grid::Direction::East,
            0,
            Count::MAX,
        )
        .unwrap()
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
                } else {
                    optimal_cells[point] = b'X';
                }
            }
        }
        optimal_cells.print();
        count
    }
}
