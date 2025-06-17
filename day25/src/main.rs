use stephen_morris_utils::timer::time;
const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
const TESTINPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

mod bit_array;
mod locks;

fn main() {
    part1();
}

fn part1() -> usize {
    let schem = time(|| locks::Schematics::parse_input(INPUT), "parse_input");
    let count = time(|| schem.count_matches(), "count_matches");
    schem.print_duration();
    count.print_all();
    *count
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 3690);
    }
}
