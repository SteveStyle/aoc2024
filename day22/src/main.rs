// #![allow(dead_code, unused)]

use secret::{most_bananas, parse_input, sum_secrets};
use stephen_morris_utils::fixed_queue;
use stephen_morris_utils::timer::time;
const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "1
10
100
2024";

#[allow(dead_code)]
const TESTINPUT2: &str = "1
2
3
2024";

mod secret;

fn main() {
    let mut secrets = time(|| parse_input(INPUT), "secrets");
    let total = time(|| sum_secrets(&mut secrets), "total");

    secrets.print_duration();
    total.print_all();

    let mut secrets = time(|| parse_input(INPUT), "secrets");
    let most_bananas = time(|| most_bananas(&mut secrets), "most_bananas");
    secrets.print_duration();
    most_bananas.print_all();
}
