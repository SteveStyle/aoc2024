#![allow(unused_imports, dead_code, unused_variables)]
pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut v = Vec::new();
    for line in input.lines() {
        v.push(Vec::<u8>::from(line.as_bytes()));
    }
    v
}

pub fn count_xmas(v: &[Vec<u8>]) -> u32 {
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

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::*;

    #[test]
    fn test_count_xmas() {
        let v = parse_input(TESTINPUT);
        assert_eq!(count_xmas(&v, "XMAS"), 18);
    }
}
