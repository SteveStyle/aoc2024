#![allow(dead_code, unused_variables, unused_mut)]
use regex::Regex;

// parse the input string to find the pattern 'mul(a,b)'.  Return each pair as a tuple in a vector.  Use a regex expression to find the pattern.
pub fn parse_input(input: &str) -> Vec<(i64, i64)> {
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

pub enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
}

enum InstructionFormatPart {
    String(&'static str),
    Number { terminator: &'static str },
}

use InstructionFormatPart as Part;

const MULFORMAT: [Part; 3] = [
    Part::String("Mul("),
    Part::Number { terminator: "," },
    Part::Number { terminator: ")" },
];
const DOFORMAT: [Part; 1] = [Part::String("Do()")];
const DONTFORMAT: [Part; 1] = [Part::String("Don't()")];

const INSTRUCTIONFORMATS: [&[Part]; 3] = [&MULFORMAT, &DOFORMAT, &DONTFORMAT];

pub fn parse_input2(input: &str) -> Vec<Instruction> {
    let ret = Vec::new();
    let mut i = 0;

    ret
}

pub fn sum_products(pairs: &[(i64, i64)]) -> i64 {
    pairs.iter().map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let pairs = parse_input(crate::TESTINPUT);
        assert_eq!(pairs.len(), 4);
        assert_eq!(pairs[0], (2, 4));
        assert_eq!(pairs[1], (5, 5));
        assert_eq!(pairs[2], (11, 8));
        assert_eq!(pairs[3], (8, 5));
    }

    #[test]
    fn test_sum_products() {
        let pairs = parse_input(crate::TESTINPUT);
        assert_eq!(sum_products(&pairs), 161);
    }

    #[test]
    fn f() {
        let (a, b, c) = (1, 2, 3);

        let g = || a + b + c;

        let d = g();

        println!("{d}");
    }
}
