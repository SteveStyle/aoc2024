use stephen_morris_utils::grid;
use stephen_morris_utils::timer;

const INPUT: &str = include_str!("input.txt");
#[allow(dead_code)]
const TESTINPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

mod guard;

fn main() {
    let mut grid = timer::time(|| guard::parse_input(INPUT), "Parse input");
    let mut guard = timer::time(|| guard::extract_guard(&mut grid), "Extract guard");
    guard.print_all();
    //grid.print_all();
    let count = timer::time(
        || guard.as_mut().unwrap().count_guard_positions(&grid),
        "Count guard positions",
    );

    grid.print_duration();
    guard.print_all();
    count.print_all();

    let mut grid = timer::time(|| guard::parse_input(INPUT), "Parse input");
    let mut guard = timer::time(|| guard::extract_guard(&mut grid), "Extract guard");
    let count = timer::time(
        || guard.as_mut().unwrap().count_blockers(&grid),
        "Count blockers",
    );
    count.print_all();
}
