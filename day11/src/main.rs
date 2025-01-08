//#![allow(dead_code, unused)]
use std::collections::HashMap;

use stephen_morris_utils::timer;

//const INPUT: &str = include_str!("input.txt");

const INPUT: &str = "0 4 4979 24 4356119 914 85734 698829";
#[allow(dead_code)]
const TESTINPUT: &str = "125 17";

mod stones;

fn main() {
    let mut fhash = HashMap::new();
    let v = timer::time(|| stones::parse_input(INPUT), "parse_input");
    let f = timer::time(|| stones::f_list(75, &v, &mut fhash), "f_list");

    v.print_duration();
    f.print_all();
}
