use stephen_morris_utils as utils;
use utils::timer;

mod parse_and_calc;
mod parser;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
#[allow(dead_code)]
const TESTINPUT2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() {
    let empty = timer::time(|| 0, "Parse empty");
    let instructions = timer::time(|| parse_and_calc::parse_input(INPUT), "Parse input");
    let sum = timer::time(
        || parse_and_calc::sum_products(&instructions),
        "Sum products",
    );
    //pairs.print_all();
    println!("{} instructions found", instructions.len());

    let sum2 = timer::time(
        || parse_and_calc::sum_products22(&instructions),
        "Sum products2",
    );

    empty.print_all();
    instructions.print_duration();
    sum.print_all();
    sum2.print_all();
}
