use std::collections::{hash_set, HashSet};

use super::grid;
use crate::grid::{Grid, Point};
pub fn parse_input(input: &str) -> Grid<u8> {
    Grid::from(input)
}
type Count = usize;
pub fn trailheads(grid: &Grid<u8>) -> Count {
    let mut count = 0;
    let mut hashset: HashSet<Point> = HashSet::new();
    for (x, y, &v) in grid {
        if v == b'0' {
            hashset.clear();
            find_heads(Point::new(x, y), b'0', &mut hashset, grid);
            //println!("trailhead at ({x},{y}) has count {}", hashset.len());
            count += hashset.len();
        }
    }
    count
}
fn find_heads(point: Point, v: u8, hashset: &mut HashSet<Point>, grid: &Grid<u8>) {
    if v == b'9' {
        hashset.insert(point);
    } else {
        for (i, j) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let Some(new_point) = grid.test_move(point, i, j) {
                if *grid.get_pos(new_point) == v + 1 {
                    find_heads(new_point, v + 1, hashset, grid);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        test_find_heads(
            "0123
1234
8765
9876",
        );
    }

    #[test]
    fn test2() {
        test_find_heads(
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        );
    }

    #[test]
    fn test3() {
        test_find_heads(
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        );
    }

    #[test]
    fn test4() {
        test_find_heads(
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        );
    }

    #[test]
    fn test() {
        test_find_heads(crate::TESTINPUT);
    }

    fn test_find_heads(input: &str) {
        let grid = super::parse_input(input);
        for y in 0..grid.height {
            for x in 0..grid.width {
                print!("{}", *grid.get(x, y) as char);
            }
            println!();
        }
        let count = super::trailheads(&grid);
        println!("count: {count}");
    }
}
