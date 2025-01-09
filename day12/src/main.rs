#![allow(dead_code, unused)]

use stephen_morris_utils::timer;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

//mod grid;
mod grid;
mod region;

fn main() {
    let mut region_map = timer::time(|| region::RegionMap::new(INPUT), "Initialise region map");
    let price = timer::time(|| region_map.price(), "Calculate price");
    let price_from_sides = timer::time(
        || region_map.price_from_sides(),
        "Calculate price from sides",
    );

    region_map.print_duration();
    price.print_all();
    price_from_sides.print_all();
}
