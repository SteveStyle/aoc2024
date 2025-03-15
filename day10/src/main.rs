use stephen_morris_utils::grid;
use stephen_morris_utils::timer;
use trailheads::trailheads2;

const INPUT: &str = include_str!("input.txt");
#[allow(dead_code)]
const TESTINPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

mod trailheads;

fn main() {
    let grid = timer::time(|| trailheads::parse_input(INPUT), "parse_input");
    let trailheads = timer::time(|| trailheads::trailheads(&grid), "trailheads");

    grid.print_duration();
    trailheads.print_all();

    let trailheads2 = timer::time(|| trailheads2(&grid), "trailheads2");

    trailheads2.print_all();
}
