#![allow(unused_imports, dead_code, unused_variables)]
pub fn parse_input1(input: &str) -> Vec<Vec<u8>> {
    let mut v = Vec::with_capacity(140);
    for line in input.lines() {
        v.push(Vec::<u8>::from(line.as_bytes()));
    }
    v
}

pub fn count_xmas1(v: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    let width = v[0].len() as i32;
    let height = v.len() as i32;
    for (start_row, line) in v.iter().enumerate() {
        for (start_col, &b) in line.iter().enumerate() {
            if b == b'X' {
                for x in [-1_i32, 0, 1] {
                    for y in [-1_i32, 0, 1] {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        let start_row = start_row as i32;
                        let start_col = start_col as i32;
                        let end_row = start_row + 3 * x;
                        let end_col = start_col + 3 * y;
                        if end_row >= 0
                            && end_row < height
                            && end_col >= 0
                            && end_col < width
                            && v[end_row as usize][end_col as usize] == b'S'
                            && v[(start_row + 2 * x) as usize][(start_col + 2 * y) as usize] == b'A'
                            && v[(start_row + x) as usize][(start_col + y) as usize] == b'M'
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

pub fn count_x_mas1(v: &[Vec<u8>]) -> u32 {
    let mut count = 0;
    let width = v[0].len() as i32;
    let height = v.len() as i32;
    for (start_row_minus_one, line) in v[1..=height as usize - 2].iter().enumerate() {
        for (start_col_minus_one, &b) in line[1..=width as usize - 2].iter().enumerate() {
            if b == b'A' {
                match [
                    v[start_row_minus_one][start_col_minus_one],
                    v[start_row_minus_one + 2][start_col_minus_one + 2],
                    v[start_row_minus_one][start_col_minus_one + 2],
                    v[start_row_minus_one + 2][start_col_minus_one],
                ] {
                    [b'M', b'S', b'M', b'S']
                    | [b'M', b'S', b'S', b'M']
                    | [b'S', b'M', b'M', b'S']
                    | [b'S', b'M', b'S', b'M'] => count += 1,
                    _ => {}
                }
            }
        }
    }

    count
}

use crate::grid::Grid;
pub fn parse_input(input: &str) -> Grid<u8> {
    Grid::from(input)
}

pub fn count_xmas(v: &Grid<u8>) -> u32 {
    let mut count = 0;
    let width = v.width as i32;
    let height = v.height as i32;
    for (start_row, start_col, value) in v {
        if *value == b'X' {
            for x in [-1_i32, 0, 1] {
                for y in [-1_i32, 0, 1] {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    let start_row = start_row as i32;
                    let start_col = start_col as i32;
                    let end_row = start_row + 3 * x;
                    let end_col = start_col + 3 * y;
                    if end_row >= 0
                        && end_row < width
                        && end_col >= 0
                        && end_col < height
                        && *v.get(end_row as usize, end_col as usize) == b'S'
                        && *v.get((start_row + 2 * x) as usize, (start_col + 2 * y) as usize)
                            == b'A'
                        && *v.get((start_row + x) as usize, (start_col + y) as usize) == b'M'
                    {
                        //println!("found {} {} {} {}", start_row, start_col, x, y);
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn count_x_mas(v: &Grid<u8>) -> u32 {
    let mut count = 0;
    let width = v.width;
    let height = v.height;
    for (start_row, start_col, value) in v {
        if start_row == 0 || start_row == height - 1 || start_col == 0 || start_col == width - 1 {
            continue;
        }
        if *value == b'A' {
            match [
                v.get(start_row - 1, start_col - 1),
                v.get(start_row + 1, start_col + 1),
                v.get(start_row - 1, start_col + 1),
                v.get(start_row + 1, start_col - 1),
            ] {
                [b'M', b'S', b'M', b'S']
                | [b'M', b'S', b'S', b'M']
                | [b'S', b'M', b'M', b'S']
                | [b'S', b'M', b'S', b'M'] => count += 1,
                _ => {}
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::*;

    #[test]
    fn test_count_xmas() {
        let v = parse_input(TESTINPUT);
        assert_eq!(count_xmas(&v), 18);
    }

    #[test]
    fn test_count_x_mas() {
        let v = parse_input(TESTINPUT);
        assert_eq!(count_x_mas(&v), 9);
    }
}
