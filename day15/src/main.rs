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
mod lanternfish;
mod lanternfish2;

fn main() {
    let mut lanternfish = timer::time(|| lanternfish::Lanternfish::new(INPUT), "Lanternfish::new");
    let gps_sum = timer::time(
        || {
            lanternfish.move_robot_fully();
            lanternfish.gps_sum()
        },
        "Lanternfish::gps_sum",
    );

    lanternfish.print_duration();
    gps_sum.print_all();

    let mut lanternfish = timer::time(|| lanternfish2::Lanternfish::new(INPUT), "Lanternfish::new");
    let gps_sum = timer::time(
        || {
            lanternfish.move_robot_fully();
            lanternfish.gps_sum()
        },
        "Lanternfish::gps_sum",
    );

    lanternfish.print_duration();
    gps_sum.print_all();
}
