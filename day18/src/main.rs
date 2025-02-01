#![allow(dead_code, unused)]

use stephen_morris_utils::timer::time;
const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

mod grid;
mod memory;

use memory::Memory;

fn main() {
    let mut memory = time(|| Memory::new(INPUT), "new");
    let min_path = time(|| memory.min_path_period(1024), "min_path_period");
    memory.print_duration();
    min_path.print_all();

    let min_path2 = time(|| memory.min_path_period2(1024), "min_path_period2");

    min_path2.print_all();

    let disconnection_point = time(|| memory.find_disconnection(), "find_disconnection");

    disconnection_point.print_all();
}

fn part1(input: &str) -> usize {
    let mut memory = Memory::new(TESTINPUT);
    memory.min_path_period(12)
}

fn part1_jobs(input: &str) -> usize {
    let mut memory = Memory::new(TESTINPUT);
    memory.min_path_period2(12)
}
fn part2(input: &str) -> grid::Point {
    let mut memory = Memory::new(TESTINPUT);
    memory.find_disconnection()
}

#[cfg(test)]
mod constants {
    pub const HEIGHT: usize = 7;
    pub const WIDTH: usize = 7;
}
#[cfg(not(test))]
mod constants {
    pub const HEIGHT: usize = 71;
    pub const WIDTH: usize = 71;
}

use constants::{HEIGHT, WIDTH};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 22);
    }
    #[test]
    fn test_part1_jobs() {
        assert_eq!(part1_jobs(TESTINPUT), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTINPUT), grid::Point { x: 6, y: 1 });
    }

    #[test]
    fn test_constants() {
        println!("height: {HEIGHT}, width: {WIDTH}");
    }
}
