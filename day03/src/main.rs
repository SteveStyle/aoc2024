use regex::Regex;
use stephen_morris_utils as utils;
use utils::timer;

mod parse_and_calc;
mod parser;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

fn main() {
    let pairs = timer::time(|| parse_and_calc::parse_input(INPUT), "Parse input");
    let sum = timer::time(|| parse_and_calc::sum_products(&pairs), "Sum products");
    //pairs.print_all();
    println!("{} pairs found", pairs.len());
    pairs.print_duration();
    sum.print_all();
}
