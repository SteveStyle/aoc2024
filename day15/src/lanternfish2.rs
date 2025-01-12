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

use super::grid::{Direction, Grid, Point, Vector};

type Count = usize;

#[derive(Debug)]
pub struct Lanternfish {
    pub grid: Grid<u8>,
    pub robot: Point,
    pub steps: Vec<Direction>,
    pub step_count: Count,
    change_log: Vec<(Point, u8)>,
}

impl Lanternfish {
    pub fn new(input: &str) -> Self {
        let mut split = input.split("\n\n");
        if let (Some(grid), Some(steps)) = (split.next(), split.next()) {
            let grid = grid.trim();
            let grid = grid.chars().fold(String::new(), |mut acc, c| {
                (match c {
                    '#' => "##",
                    'O' => "[]",
                    '@' => "@.",
                    '.' => "..",
                    '\n' => "\n",
                    _ => panic!("Lanternfish::new() could not parse the grid."),
                })
                .chars()
                .for_each(|c| acc.push(c));
                acc
            });
            let grid: Grid<u8> = (&grid[..]).into();

            let steps = steps.chars().filter_map(Direction::from_char).collect();
            let robot = grid.find(b'@').unwrap();
            Self {
                grid,
                robot,
                steps,
                step_count: 0,
                change_log: Vec::new(),
            }
        } else {
            panic!("Lanternfish::new() could not split the input.");
        }
    }

    pub fn move_cell(&mut self, this_cell_point: Point, direction: Direction) -> bool {
        /// move cell moves the value of the this cell into the next cell.  If the next cell is occupied it requests that that cell is also moved.  
        /// If the next cell is part of a box it also moves the other part of the box, unless the boxes are aligned.
        let this_cell_value = self.grid[this_cell_point];
        let next_cell_point = (this_cell_point + Vector::from(direction)).unwrap();
        let next_cell_value = self.grid[next_cell_point];
        match this_cell_value {
            b'#' => {
                panic!("attempt to move a cell containing #")
            }
            b'[' if (direction == Direction::Down || direction == Direction::Up) => {
                match next_cell_value {
                    b'.' => {
                        self.change_log.push((next_cell_point, b'['));
                        true
                    }
                    b']' => {
                        self.change_log.push((next_cell_point, b'['));
                        self.move_cell(next_cell_point, direction)
                            && self.move_cell(
                                (next_cell_point + Vector::from(Direction::Left)).unwrap(),
                                direction,
                            )
                    }
                    b'[' => {
                        self.change_log.push((next_cell_point, b'['));
                        self.move_cell(next_cell_point, direction)
                    }
                    b'#' => false,
                    _ => {
                        panic!("unkown cell value")
                    }
                }
            }
            b']' if (direction == Direction::Down || direction == Direction::Up) => {
                match next_cell_value {
                    b'.' => {
                        self.change_log.push((next_cell_point, b']'));
                        true
                    }
                    b'[' => {
                        self.change_log.push((next_cell_point, b']'));
                        self.move_cell(next_cell_point, direction)
                            && self.move_cell(
                                (next_cell_point + Vector::from(Direction::Right)).unwrap(),
                                direction,
                            )
                    }
                    b']' => {
                        self.change_log.push((next_cell_point, b']'));
                        self.move_cell(next_cell_point, direction)
                    }
                    b'#' => false,
                    _ => {
                        panic!("unkown cell value")
                    }
                }
            }
            b'@' | b'[' | b']' => {
                if self.move_cell(next_cell_point, direction) {
                    self.change_log.push((next_cell_point, this_cell_value));
                    true
                } else {
                    false
                }
            }
            _ => {
                self.change_log.push((next_cell_point, this_cell_value));
                true
            }
        }
    }

    pub fn move_robot(&mut self) -> bool {
        self.change_log.clear();
        if self.step_count < self.steps.len() {
            if self.move_cell(self.robot, self.steps[self.step_count]) {
                self.change_log.push((self.robot, b'.'));
                self.robot = (self.robot + Vector::from(self.steps[self.step_count])).unwrap();
                for (p, v) in self.change_log.iter() {
                    self.grid[*p] = *v;
                }
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
        assert_eq!(lanternfish.grid[lanternfish.robot], b'@');
        assert_eq!(lanternfish.steps.len(), 15);
    }

    #[test]
    fn test_lanternfish_move_cell() {
        let mut lanternfish = Lanternfish::new(TESTINPUT3);
        lanternfish.grid.print();
        assert!(lanternfish.move_robot());
        lanternfish.grid.print();
        assert!(lanternfish.move_robot());
        lanternfish.grid.print();
        assert!(lanternfish.move_robot());
        lanternfish.grid.print();
        assert_eq!(lanternfish.grid[lanternfish.robot], b'@');
        assert_eq!(lanternfish.robot, Point::new(9, 5));
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
