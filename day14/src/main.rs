#![allow(dead_code, unused)]

use stephen_morris_utils::timer;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

mod grid;
mod restroom;

fn main() {
    let mut restroom = timer::time(|| restroom::Restroom::new(INPUT), "Parsing input");
    let safety_factor = timer::time(
        || restroom.safety_factor_at_time(100),
        "Calculating safety factor",
    );

    let min_safety_factor_time = timer::time(
        || restroom.print_at_minimum_safety_factor(10000),
        "Finding minimum safety factor",
    );
    restroom.print_duration();
    safety_factor.print_all();
    min_safety_factor_time.print_all();
}
