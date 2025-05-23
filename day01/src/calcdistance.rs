#![allow(dead_code)]
use std::collections::{BTreeMap, HashMap};

#[allow(dead_code)]
const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut x = Vec::new();
    let mut y = Vec::new();
    for line in input.lines() {
        let mut it = line.split_whitespace().map(|n| n.parse());
        if let (Some(Ok(xx)), Some(Ok(yy))) = (it.next(), it.next()) {
            x.push(xx);
            y.push(yy);
        }
    }
    (x, y)
}

pub fn calc_distance(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
    a.sort();
    b.sort();

    let ret = a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum();
    ret
}

fn vec_to_count_map(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::with_capacity(500);

    for i in v {
        let sum = map.entry(i).or_insert(0);
        *sum += 1;
    }

    map
}
fn vec_to_sum_map(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::with_capacity(500);

    for i in v {
        let sum = map.entry(i).or_insert(0);
        *sum += i;
    }

    map
}

pub fn count_repeats1(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let b_sums = vec_to_sum_map(b);
    let ret = a.iter().map(|i| b_sums.get(i).unwrap_or(&0)).sum();

    ret
}

pub fn count_repeats2(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let a_counts = vec_to_count_map(a);
    let b_sums = vec_to_sum_map(b);
    let ret = a_counts
        .iter()
        .map(|(i, count)| b_sums.get(i).unwrap_or(&0) * count)
        .sum();

    ret
}

pub fn count_repeats3(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
    a.sort();
    b.sort();

    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();

    let mut ret = 0;

    let mut a_next = a_iter.next();
    let mut b_next = b_iter.next();

    while let (Some(a_val), Some(b_val)) = (a_next, b_next) {
        match a_val.cmp(&b_val) {
            std::cmp::Ordering::Equal => {
                let a_count = {
                    let mut count = 1;
                    loop {
                        a_next = a_iter.next();
                        if a_next == Some(a_val) {
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    count
                };
                let b_sum = {
                    let mut sum = b_val;
                    loop {
                        b_next = b_iter.next();
                        if b_next == Some(b_val) {
                            sum += b_val;
                        } else {
                            break;
                        }
                    }
                    sum
                };
                ret += a_count * b_sum;
            }
            std::cmp::Ordering::Less => {
                a_next = a_iter.next();
            }
            std::cmp::Ordering::Greater => {
                b_next = b_iter.next();
            }
        }
    }

    ret
}

fn vec_to_btreemap(v: Vec<i32>) -> BTreeMap<i32, i32> {
    let mut map = BTreeMap::new();

    for i in v {
        let sum = map.entry(i).or_insert(0);
        *sum += i;
    }

    map
}

pub fn count_repeats4(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let b_sums = vec_to_btreemap(b);
    let ret = a.iter().map(|i| b_sums.get(i).unwrap_or(&0)).sum();

    ret
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let (a, b) = parse_input(EXAMPLE);
        let distance = calc_distance(a, b);
        assert_eq!(distance, 11);
    }
    #[test]
    fn test_count_repeats() {
        let (a, b) = parse_input(EXAMPLE);
        let repeats = count_repeats1(a, b);
        assert_eq!(repeats, 31);
    }
}
