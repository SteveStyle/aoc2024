#![allow(dead_code, unused_variables, unused_mut)]
use crate::parser::{Instruction, ParseIterator};

pub fn parse_input(input: &str) -> Vec<Instruction> {
    ParseIterator::new(input).collect()
}

pub fn sum_products(pairs: &[Instruction]) -> i64 {
    pairs
        .iter()
        .filter_map(|i| match i {
            Instruction::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum()
}

pub fn sum_products2(input: &Vec<Instruction>) -> i64 {
    let mut sum = 0;
    let mut enabled: bool = true;
    for instruction in input {
        match instruction {
            Instruction::Mul(a, b) => {
                if enabled {
                    sum += a * b;
                }
            }
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_products2() {
        let v = parse_input(crate::TESTINPUT2);
        let sum = sum_products2(&v);
        assert_eq!(sum, 48);
    }

    #[test]
    fn test_parse_input() {
        let v = parse_input(crate::TESTINPUT2);
        assert_eq!(v.len(), 6);
        assert_eq!(v[0], Instruction::Mul(2, 4));
        assert_eq!(v[1], Instruction::Dont);
        assert_eq!(v[2], Instruction::Mul(5, 5));
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
