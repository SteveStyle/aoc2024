use stephen_morris_utils::timer;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

mod count_xmas;
mod grid;

fn main() {
    let v = timer::time(|| count_xmas::parse_input1(INPUT), "parse_input");
    let count = timer::time(|| count_xmas::count_xmas1(&v), "count_xmas");
    let count2 = timer::time(|| count_xmas::count_x_mas1(&v), "count_x_mas");

    v.print_duration();
    count.print_all();
    count2.print_all();

    let v = timer::time(|| count_xmas::parse_input(INPUT), "parse_input");
    let count = timer::time(|| count_xmas::count_xmas(&v), "count_xmas");
    let count2 = timer::time(|| count_xmas::count_x_mas(&v), "count_x_mas");

    v.print_duration();
    count.print_all();
    count2.print_all();
}
