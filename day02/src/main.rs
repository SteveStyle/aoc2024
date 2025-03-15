use stephen_morris_utils as utils;
use utils::timer;

mod check_levels;

const TESTINPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut reports = timer::time(|| check_levels::parse_input(INPUT), "parse_input");
    let count = timer::time(
        || check_levels::check_reports(&mut reports),
        "check_reports",
    );

    let count2 = timer::time(
        || check_levels::check_reports2(&mut reports),
        "check_reports2",
    );

    reports.print_duration();
    count.print_all();
    count2.print_all();
}
