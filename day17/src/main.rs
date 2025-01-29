#![allow(dead_code, unused)]

use computer::Computer;
use stephen_morris_utils::timer::time;
const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

const TESTINPUT2: &str = "Register A: 121024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

const TESTINPUT3: &str = "Register A: 107416870455451
Register B: 0
Register C: 0

Program: 2,4,1,5,7,5,1,6,4,2,5,5,0,3,3,0";
mod computer;

fn main() {
    let mut computer = time(|| Computer::new(INPUT), "computer");
    let output = time(|| computer.execute_program(), "execute program");

    computer.print_duration();
    output.print_all();

    let find_a = time(|| computer.find_initial_a2(), "find_a");
    find_a.print_all();
}

fn part1(input: &str) -> String {
    let mut computer = Computer::new(input);
    computer.execute_program()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), "4,6,3,5,6,3,5,2,1,0");
    }
}
