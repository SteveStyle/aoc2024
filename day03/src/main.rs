use regex::Regex;
use stephen_morris_utils as utils;
use utils::timer;

const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
const TESTINPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

fn main() {
    let pairs = timer::time(|| parse_input(INPUT), "Parse input");
    let sum = timer::time(|| sum_products(&pairs), "Sum products");
    //pairs.print_all();
    println!("{} pairs found", pairs.len());
    pairs.print_duration();
    sum.print_all();
}

// parse the input string to find the pattern 'mul(a,b)'.  Return each pair as a tuple in a vector.  Use a regex expression to find the pattern.
fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            (
                cap[1].parse::<i64>().unwrap(),
                cap[2].parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn sum_products(pairs: &[(i64, i64)]) -> i64 {
    pairs.iter().map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let pairs = parse_input(TESTINPUT);
        assert_eq!(pairs.len(), 4);
        assert_eq!(pairs[0], (2, 4));
        assert_eq!(pairs[1], (5, 5));
        assert_eq!(pairs[2], (11, 8));
        assert_eq!(pairs[3], (8, 5));
    }

    #[test]
    fn test_sum_products() {
        let pairs = parse_input(TESTINPUT);
        assert_eq!(sum_products(&pairs), 161);
    }
}
