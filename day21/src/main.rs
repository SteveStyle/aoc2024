// #![allow(dead_code, unused)]

use stephen_morris_utils::grid;
use stephen_morris_utils::timer::time;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "029A
980A
179A
456A
379A";

mod keypads;

use keypads::Scenario;

fn main() {
    let mut scenario = time(|| Scenario::new(INPUT, 3), "Scenario::new()");
    let cost = time(|| scenario.cost_for_targets(), "cost");

    scenario.print_duration();
    cost.print_all();

    let mut scenario = time(|| Scenario::new(INPUT, 26), "Scenario::new()");
    let cost = time(|| scenario.cost_for_targets(), "cost");

    scenario.print_duration();
    cost.print_all();
}
