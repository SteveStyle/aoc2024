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

fn main() {
    let v = timer::time(|| count_xmas::parse_input(INPUT), "parse_input");
    let count = timer::time(|| count_xmas::count_xmas(&v), "count_xmas");

    v.print_duration();
    count.print_all();
}
