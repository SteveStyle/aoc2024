#![allow(dead_code, unused)]

use stephen_morris_utils::timer;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

const TESTINPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

mod grid;
mod maze;

fn main() {
    let mut maze = timer::time(|| maze::Maze::new(INPUT), "Maze::new");
    let res = timer::time(|| maze.minimum_score(), "Maze::minimum_score");
    let best_paths = timer::time(|| maze.find_path_cells(), "find_best_paths");

    maze.print_duration();
    res.print_all();
    best_paths.print_all();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = maze::part1(TESTINPUT);
        assert_eq!(res, 7036);
    }

    #[test]
    fn test2() {
        let res = maze::part1(TESTINPUT2);
        assert_eq!(res, 11048);
    }

    #[test]
    fn test3() {
        let res = maze::part2(TESTINPUT);
        assert_eq!(res, 45);
    }
    #[test]
    fn test4() {
        let res = maze::part2(TESTINPUT2);
        assert_eq!(res, 64);
    }
}
