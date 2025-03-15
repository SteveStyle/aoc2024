#![allow(dead_code, unused)]

use stephen_morris_utils::grid;
use stephen_morris_utils::timer::time;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

mod towels;

use towels::TowelWords;

fn main() {
    let mut towel_words = time(|| TowelWords::new(INPUT), "new");
    let minimal_list = time(|| towel_words.build_minimal_list(), "minimal list");
    let count = time(|| towel_words.count_possible_targets(), "count");

    towel_words.print_duration();
    minimal_list.print_duration();
    count.print_all();

    let ways = time(|| towel_words.count_possible_ways(), "ways");

    ways.print_all();
}

fn part1(input: &str) -> usize {
    let mut towel_words = TowelWords::new(input);
    towel_words.build_minimal_list();
    towel_words.count_possible_targets()
}

fn part2(input: &str) -> usize {
    let mut towel_words = TowelWords::new(input);
    towel_words.build_minimal_list();
    towel_words.count_possible_ways()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTINPUT), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTINPUT), 16);
    }
}
