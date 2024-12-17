use num_traits::{ConstOne, ConstZero, Num};

trait ParseItemSteve<'a, T> {
    fn parse_item(slice: &'a str, term: &str) -> Option<(&'a str, T)> {
        None
    }
}

impl<'a, T> ParseItemSteve<'a, T> for T
where
    T: Num + ConstZero + ConstOne,
{
    fn parse_item(slice: &'a str, term: &str) -> Option<(&'a str, T)> {
        None
    }
}

impl<'a> ParseItemSteve<'a, bool> for bool {
    fn parse_item(slice: &'a str, term: &str) -> Option<(&'a str, bool)> {
        if slice.starts_with(term) {
            Some((&slice[term.len()..], true))
        } else {
            None
        }
    }
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

const MULFORMATPARTS: [Part; 3] = [
    Part::String("Mul("),
    Part::Number { terminator: "," },
    Part::Number { terminator: ")" },
];
const DOFORMATPARTS: [Part; 1] = [Part::String("Do()")];
const DONTFORMATPARTS: [Part; 1] = [Part::String("Don't()")];

const INSTRUCTIONFORMATPARTS: [&[Part]; 3] = [&MULFORMATPARTS, &DOFORMATPARTS, &DONTFORMATPARTS];

struct InstructionFormat {
    parts: &'static [Part],
}

const MULLFORMAT: InstructionFormat = InstructionFormat {
    parts: &MULFORMATPARTS,
};

const DOFORMAT: InstructionFormat = InstructionFormat {
    parts: &DOFORMATPARTS,
};

const DONTFORMAT: InstructionFormat = InstructionFormat {
    parts: &DONTFORMATPARTS,
};

const INSTRUCTIONFORMATS: [InstructionFormat; 3] = [MULLFORMAT, DOFORMAT, DONTFORMAT];

impl InstructionFormat {
    fn parse_value(slice: &mut &str) -> Option<Instruction> {}
}

pub fn parse_input2(input: &str) -> Vec<Instruction> {
    let ret = Vec::new();
    let mut i = 0;

    ret
}
