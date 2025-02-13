// #![allow(dead_code, unused)]

use secret::{parse_input, sum_secrets};
use stephen_morris_utils::timer::time;
const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "1
10
100
2024";

mod grid;
mod secret;

fn main() {
    let mut secrets = time(|| parse_input(INPUT), "secrets");
    let total = time(|| sum_secrets(&mut secrets), "total");

    secrets.print_duration();
    total.print_all();
}
