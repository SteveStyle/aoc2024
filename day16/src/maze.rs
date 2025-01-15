use crate::grid::{Direction, Grid, Point, Vector};

type Count = u32;

pub fn part1(input: &str) -> Count {
    Maze::new(input).minimum_score()
}
pub fn part2(input: &str) -> Count {
    Maze::new(input).find_path_cells()
}

#[derive(Debug)]
pub struct Maze {
    maze: Grid<u8>,
    counts: Grid<Option<Count>>,
    best_path: Grid<bool>,
    start: Point,
    end: Point,
}

impl Maze {
    pub fn new(input: &str) -> Self {
        let maze: Grid<u8> = input.into();
        let counts = Grid::new(maze.width, maze.height, None);
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

    pub fn find_path_cells(&mut self) -> Count {
        fn move_forward(
            maze: &Grid<u8>,
            best_path: &mut Grid<bool>,
            current_cell: Point,
            current_direction: Direction,
            score: Count,
            best_score: Count,
        ) -> bool {
            let mut res = false;
            for (new_direction, new_score) in [
                (current_direction, score + 1),
                (current_direction.left(), score + 1001),
                (current_direction.right(), score + 1001),
            ] {
                if new_score <= best_score {
                    if let Some(next_cell) =
                        maze.add_vector(current_cell, Vector::from(new_direction))
                    {
                        match maze[next_cell] {
                            b'.' => {
                                if new_score < best_score
                                    && move_forward(
                                        maze,
                                        best_path,
                                        next_cell,
                                        new_direction,
                                        new_score,
                                        best_score,
                                    )
                                {
                                    best_path[next_cell] = true;
                                    res = true;
                                }
                            }
                            b'E' => {
                                if new_score == best_score {
                                    res = true;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            res
        }
        self.best_path[self.start] = true;
        self.best_path[self.end] = true;
        let minimum_score = self.minimum_score();
        move_forward(
            &self.maze,
            &mut self.best_path,
            self.start,
            Direction::Right,
            0,
            minimum_score,
        );
        self.best_path.iter().map(|(_, &b)| b as Count).sum()
    }

    pub fn populate_counts(&mut self) {
        fn move_forward(
            maze: &Grid<u8>,
            counts: &mut Grid<Option<Count>>,
            current_cell: Point,
            current_direction: Direction,
            score: Count,
        ) {
            for (new_direction, new_score) in [
                (current_direction, score + 1),
                (current_direction.left(), score + 1001),
                (current_direction.right(), score + 1001),
            ] {
                if let Some(next_cell) = maze.add_vector(current_cell, Vector::from(new_direction))
                {
                    match maze[next_cell] {
                        b'.' => match counts[next_cell] {
                            None => {
                                counts[next_cell] = Some(new_score);
                                move_forward(maze, counts, next_cell, new_direction, new_score);
                            }
                            Some(previous_score) if previous_score > new_score => {
                                counts[next_cell] = Some(new_score);
                                move_forward(maze, counts, next_cell, new_direction, new_score);
                            }
                            _ => {}
                        },
                        b'E' => match counts[next_cell] {
                            None => {
                                counts[next_cell] = Some(new_score);
                            }
                            Some(previous_score) if previous_score > new_score => {
                                counts[next_cell] = Some(new_score);
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
        self.counts[self.start] = Some(0);
        move_forward(
            &self.maze,
            &mut self.counts,
            self.start,
            Direction::Right,
            0,
        );
    }

    pub fn minimum_score(&mut self) -> Count {
        self.populate_counts();
        self.counts[self.end].unwrap()
    }
}
