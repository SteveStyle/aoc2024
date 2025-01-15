#[allow(dead_code)]
const TESTINPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

#[allow(dead_code)]
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
            let grid: Grid<u8> = grid.into();
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

    pub fn move_cell(&mut self, cell: Point, direction: Direction) -> bool {
        let this_cell = self.grid[cell];
        let next_cell_point = (cell + Vector::from(direction)).unwrap();
        let next_cell = self.grid[next_cell_point];
        match next_cell {
            b'#' => false,
            b'O' => {
                if self.move_cell(next_cell_point, direction) {
                    self.grid[next_cell_point] = this_cell;
                    true
                } else {
                    false
                }
            }
            _ => {
                self.grid[next_cell_point] = this_cell;
                true
            }
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
            .filter_map(|(p, &v)| if v == b'O' { Some(Self::gps(p)) } else { None })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lanternfish_new() {
        let lanternfish = Lanternfish::new(TESTINPUT);
        assert_eq!(lanternfish.grid[lanternfish.robot], b'@');
        assert_eq!(lanternfish.steps.len(), 15);
    }

    #[test]
    fn test_lanternfish_move_cell() {
        let mut lanternfish = Lanternfish::new(TESTINPUT);
        lanternfish.grid.print();
        assert!(lanternfish.move_robot());
        lanternfish.grid.print();
        assert!(lanternfish.move_robot());
        lanternfish.grid.print();
        assert!(lanternfish.move_robot());
        lanternfish.grid.print();
        assert_eq!(lanternfish.grid[lanternfish.robot], b'@');
        assert_eq!(lanternfish.robot, Point::new(2, 1));
    }

    #[test]
    fn test_lanternfish_move_robot_fully1() {
        let mut lanternfish = Lanternfish::new(TESTINPUT);
        lanternfish.move_robot_fully();
        assert_eq!(lanternfish.grid[lanternfish.robot], b'@');
        assert_eq!(lanternfish.gps_sum(), 2028);
    }

    #[test]
    fn test_lanternfish_move_robot_fully2() {
        let mut lanternfish = Lanternfish::new(TESTINPUT2);
        lanternfish.move_robot_fully();
        assert_eq!(lanternfish.grid[lanternfish.robot], b'@');
        assert_eq!(lanternfish.gps_sum(), 10092);
    }
}
