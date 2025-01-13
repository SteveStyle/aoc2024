const TESTINPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

const TESTINPUT2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

const TESTINPUT3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

use core::panic;

use super::grid::{Direction, Grid, Point, Vector};

type Count = usize;

#[derive(Debug)]
pub struct Lanternfish {
    pub grid: Grid<u8>,
    pub robot: Point,
    pub steps: Vec<Direction>,
    pub step_count: Count,
}

impl Lanternfish {
    pub fn new(input: &str) -> Self {
        let mut split = input.split("\n\n");
        if let (Some(grid), Some(steps)) = (split.next(), split.next()) {
            let grid = grid.trim();
            let grid = grid.bytes().fold(Vec::new(), |mut acc, c| {
                acc.extend(
                    match c {
                        b'#' => "##",
                        b'O' => "[]",
                        b'@' => "@.",
                        b'.' => "..",
                        b'\n' => "\n",
                        _ => panic!("Lanternfish::new() could not parse the grid."),
                    }
                    .bytes(),
                );
                acc
            });
            let grid: Grid<u8> = (&grid[..]).into();

            let steps = steps.chars().filter_map(Direction::try_from_char).collect();
            let robot = grid.find(b'@').unwrap();
            Self {
                grid,
                robot,
                steps,
                step_count: 0,
            }
        } else {
            panic!("Lanternfish::new() could not split the input.");
        }
    }

    pub fn move_cell_horizontal(&mut self, this_cell_point: Point, direction: Direction) -> bool {
        assert!(direction == Direction::Left || direction == Direction::Right);
        /// move cell moves the value of the this cell into the next cell.  If the next cell is occupied it requests that that cell is also moved.  
        /// If the next cell is wall it returns false.
        let this_cell_value = self.grid[this_cell_point];
        let next_cell_point = (this_cell_point + Vector::from(direction)).unwrap();
        //let next_cell_value = self.grid[next_cell_point];
        match this_cell_value {
            b'[' | b']' | b'@' => {
                if self.move_cell_horizontal(next_cell_point, direction) {
                    self.grid[next_cell_point] = this_cell_value;
                    true
                } else {
                    false
                }
            }
            b'.' => {
                // should not happen
                true
            }
            b'#' => false,
            _ => {
                panic!("unkown cell value")
            }
        }
    }

    pub fn move_cell_vertical(
        &mut self,
        this_row_points_to_move: Vec<Point>,
        direction: Direction,
    ) -> bool {
        assert!(direction == Direction::Up || direction == Direction::Down);
        if this_row_points_to_move.len() == 0 {
            return true;
        }
        let next_row_points_to_move =
            this_row_points_to_move
                .iter()
                .try_fold(Vec::new(), |mut acc, &p| {
                    match self.grid[p] {
                        b'.' => {}
                        b'#' => {
                            return Err(());
                        }

                        b'[' => {
                            if let Some(new_p) = (p + Vector::from(direction)) {
                                acc.push(new_p);
                            }
                            if let Some(new_p) =
                                (p + (Vector::from(direction) + Vector::from(Direction::Right)))
                            {
                                acc.push(new_p);
                            }
                        }
                        b']' => {
                            if let Some(new_p) = (p + Vector::from(direction)) {
                                acc.push(new_p);
                            }
                            if let Some(new_p) =
                                (p + (Vector::from(direction) + Vector::from(Direction::Left)))
                            {
                                acc.push(new_p);
                            }
                        }
                        b'@' => {
                            if let Some(new_p) = (p + Vector::from(direction)) {
                                acc.push(new_p);
                            }
                        }
                        _ => panic!("unkown cell value"),
                    };
                    Ok(acc)
                });

        if let Ok(next_row_points_to_move) = next_row_points_to_move {
            if self.move_cell_vertical(next_row_points_to_move, direction) {
                this_row_points_to_move.iter().for_each(|&p| {
                    self.grid[(p + Vector::from(direction)).unwrap()] = self.grid[p];
                });
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn move_cell(&mut self, this_cell_point: Point, direction: Direction) -> bool {
        if direction == Direction::Left || direction == Direction::Right {
            self.move_cell_horizontal(this_cell_point, direction)
        } else {
            self.move_cell_vertical(vec![this_cell_point], direction)
        }
    }
    pub fn move_robot(&mut self) -> bool {
        if self.step_count < self.steps.len() {
            if self.move_cell(self.robot, self.steps[self.step_count]) {
                self.grid[self.robot] = b'.';
                self.robot = (self.robot + Vector::from(self.steps[self.step_count])).unwrap();
            }
            self.step_count += 1;
            true
        } else {
            false
        }
    }

    pub fn move_robot_fully(&mut self) {
        while self.move_robot() {}
    }

    pub fn gps(p: Point) -> usize {
        100 * p.y + p.x
    }

    pub fn gps_sum(&self) -> usize {
        self.grid
            .iter()
            .filter_map(|(p, &v)| if v == b'[' { Some(Self::gps(p)) } else { None })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lanternfish_new() {
        let lanternfish = Lanternfish::new(TESTINPUT3);
        lanternfish.grid.print();
        assert_eq!(lanternfish.grid[lanternfish.robot], b'@');
        assert_eq!(lanternfish.steps.len(), 11);
    }

    #[test]
    fn test_lanternfish_move_cell() {
        let mut lanternfish = Lanternfish::new(TESTINPUT3);
        lanternfish.grid.print();
        assert!(lanternfish.move_robot());
        lanternfish.grid.print();
        assert_eq!(lanternfish.robot, Point { x: 9, y: 3 });
        assert!(lanternfish.move_robot());
        lanternfish.grid.print();
        assert_eq!(lanternfish.robot, Point { x: 9, y: 4 });
        assert!(lanternfish.move_robot());
        lanternfish.grid.print();
        assert_eq!(lanternfish.robot, Point::new(9, 5));
        assert_eq!(lanternfish.grid[lanternfish.robot], b'@');
    }

    #[test]
    fn test_lanternfish_move_robot_fully3() {
        let mut lanternfish = Lanternfish::new(TESTINPUT3);
        lanternfish.move_robot_fully();
        assert_eq!(lanternfish.grid[lanternfish.robot], b'@');
        lanternfish.grid.print();
        assert_eq!(lanternfish.robot, Point::new(5, 2));

        //assert_eq!(lanternfish.gps_sum(), 9021);
    }
    #[test]
    fn test_lanternfish_move_robot_fully2() {
        let mut lanternfish = Lanternfish::new(TESTINPUT2);
        lanternfish.move_robot_fully();
        assert_eq!(lanternfish.grid[lanternfish.robot], b'@');
        lanternfish.grid.print();
        assert_eq!(lanternfish.gps_sum(), 9021);
    }

    #[test]
    fn test_first_n_steps() {
        let mut lanternfish = Lanternfish::new(TESTINPUT2);
        for i in 0..20 {
            lanternfish.move_robot();
        }
        lanternfish.grid.print();
        for i in 0..15 {
            lanternfish.move_robot();
            println!(
                "after step : {}, direction {:?}",
                lanternfish.step_count,
                lanternfish.steps[lanternfish.step_count - 1]
            );
            lanternfish.grid.print();
        }
    }
}
