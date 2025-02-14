#![allow(dead_code, unused)]
use stephen_morris_utils::timer;

const INPUT: &str = include_str!("input.txt");
#[allow(dead_code)]
const TESTINPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

mod antinodes;

mod grid;

fn main() {
    let antenna_map = timer::time(|| antinodes::parse_input(INPUT), "parse_input");
    let count = timer::time(
        || antinodes::count_antinodes(&antenna_map),
        "count_antinodes",
    );
    let count2 = timer::time(
        || antinodes::count_antinodes2(&antenna_map),
        "count_antinodes2",
    );

    antenna_map.print_duration();
    count.print_all();
    count2.print_all();
}
