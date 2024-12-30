#![allow(dead_code, unused)]
use stephen_morris_utils::timer;

const INPUT: &str = include_str!("input.txt");
#[allow(dead_code)]
const TESTINPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

mod operators;

fn main() {
    let equations = timer::time(|| operators::parse_input(crate::INPUT), "parse_input");
    let sum = timer::time(|| operators::solve_and_sum(&equations), "solve_and_sum");
    let sum2 = timer::time(|| operators::solve_and_sum2(&equations), "solve_and_sum2");

    equations.print_duration();
    sum.print_all();
    sum2.print_all();
}
