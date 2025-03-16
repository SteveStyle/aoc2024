//Enhanced ParseItem Trait Implementation

use num_traits::{ConstOne, ConstZero, Num, PrimInt};
use std::ops::Neg;
use std::str::FromStr;

/// A trait for parsing items from a string slice
trait ParseItemTerm<'a, T> {
    /// Attempts to parse an item of type T from the start of the slice
    /// Returns Some((remaining_slice, parsed_value)) if successful, None otherwise
    fn parse_item(slice: &'a str, term: &str) -> Option<(&'a str, T)>;
}

trait ParseItem<'a, T> {
    /// Attempts to parse an item of type T from the start of the slice
    /// Returns Some((remaining_slice, parsed_value)) if successful, None otherwise
    fn parse_item(slice: &'a str) -> Option<(&'a str, T)>;
}

// Implementation for a number type that parses until terminator
impl<'a, T> ParseItemTerm<'a, T> for T
where
    T: Num + Neg<Output = T> + PrimInt + ConstZero + ConstOne + FromStr,
{
    fn parse_item(slice: &'a str, term: &str) -> Option<(&'a str, T)> {
        if slice.is_empty() {
            return None;
        }
        let mut sign = T::one();
        let mut chars = slice.char_indices();
        let mut next = chars.next();
        match next {
            Some((_, c)) if c == '-' || c == '+' => {
                if c == '-' {
                    sign = -T::one();
                }
                next = chars.next();
            }
            _ => {}
        }

        let (_, char) = next?;
        let mut number: T = sign * T::from(char.to_digit(10)?)?;
        next = chars.next();
        while let Some((_, c)) = next {
            if c.is_ascii_digit() {
                number = number * T::from(10).unwrap() + T::from(c.to_digit(10).unwrap()).unwrap();
            } else {
                break;
            }
            next = chars.next();
        }

        match next {
            Some((i, _)) => Some((slice[i..].strip_prefix(term)?, number)),
            None => term.is_empty().then_some((&slice[slice.len()..], number)),
        }
    }
}

struct StringMatches;

impl<'a> ParseItemTerm<'a, StringMatches> for StringMatches {
    fn parse_item(slice: &'a str, term: &str) -> Option<(&'a str, StringMatches)> {
        //calls to this function can sometimes be replaced, more conveniently, by the line below
        slice.strip_prefix(term).map(|s| (s, StringMatches))
    }
}

/// implement ParseItem for an Instruction enum
///
impl<'a> ParseItem<'a, Instruction> for Instruction {
    fn parse_item(slice: &'a str) -> Option<(&'a str, Instruction)> {
        fn parse_item_do(slice: &str) -> Option<(&str, Instruction)> {
            StringMatches::parse_item(slice, "do()").map(|(s, _)| (s, Instruction::Do))
        }
        fn parse_item_dont(slice: &str) -> Option<(&str, Instruction)> {
            StringMatches::parse_item(slice, "don't()").map(|(s, _)| (s, Instruction::Dont))
        }
        fn parse_item_mul(slice: &str) -> Option<(&str, Instruction)> {
            let (rest, _) = StringMatches::parse_item(slice, "mul(")?;
            let (rest, a) = i64::parse_item(rest, ",")?;
            let (rest, b) = i64::parse_item(rest, ")")?;
            Some((rest, Instruction::Mul(a, b)))
        }

        let slice = slice.trim_start();
        if let Some((slice, instruction)) = parse_item_do(slice) {
            Some((slice, instruction))
        } else if let Some((slice, instruction)) = parse_item_dont(slice) {
            Some((slice, instruction))
        } else if let Some((slice, instruction)) = parse_item_mul(slice) {
            Some((slice, instruction))
        } else {
            None
        }
    }
}

pub struct ParseIterator<'a> {
    slice: &'a str,
}

impl<'a> ParseIterator<'a> {
    pub fn new(slice: &'a str) -> Self {
        Self { slice }
    }
}

impl Iterator for ParseIterator<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice = self.slice.trim_start();
        while !self.slice.is_empty() {
            match Instruction::parse_item(self.slice) {
                Some((rest, instruction)) => {
                    self.slice = rest;
                    return Some(instruction);
                }
                None => self.slice = self.slice[1..].trim_start(),
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_item() {
        let input = "mul(1,2) do() don't()";
        let mut iter = ParseIterator::new(input);

        assert_eq!(iter.next(), Some(Instruction::Mul(1, 2)));
        assert_eq!(iter.next(), Some(Instruction::Do));
        assert_eq!(iter.next(), Some(Instruction::Dont));
        assert_eq!(iter.next(), None);
    }
}
