#![allow(dead_code, unused)]

use stephen_morris_utils::timer::time;
const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "029A
980A
179A
456A
379A";

mod grid;
mod keypads;

use keypads::Keypad;

fn main() {}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // assert_eq!(part1(TESTINPUT), 8);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(TESTINPUT), 16);
    }
}
