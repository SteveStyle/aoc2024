#![allow(dead_code, unused)]

use stephen_morris_utils::grid;
use stephen_morris_utils::timer::time;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

mod race;

use race::Race;

fn main() {
    let mut race = time(|| Race::new(INPUT), "new");
    let cheats = time(|| race.find_cheats(), "cheats");
    let count = time(|| race.count_cheats_over(100), "count");

    race.print_duration();
    cheats.print_duration();
    count.print_all();

    let count_long = time(|| race.count_long_cheats(100), "long");

    count_long.print_all();
}

fn part1(input: &str) -> usize {
    // for testing use a minimum saving of 12 picoseconds, giving 8
    let mut race = Race::new(TESTINPUT);
    race.find_cheats();
    race.count_cheats_over(12)
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 8);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(TESTINPUT), 16);
    }
}
