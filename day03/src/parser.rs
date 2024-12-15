struct FullParseInstruction<'a> {
    slice: &'a str,
    parsers: &[ParseInstruction],
}

impl<'a> FullParseInstruction<'a> {
    fn new(slice: &str) -> FullParseInstruction {
        FullParseInstruction { slice }
    }
    fn remaining_slice(&'a self) -> &'a str {
        self.slice
    }
}

trait Parse<I> {
    fn parse_once(slice: &str) -> (Option<I>, &str);
}
struct ParseInstruction {
    pattern: InstructionFormat,
}

impl ParseInstruction {
    fn new(pattern: InstructionFormat) -> ParseInstruction {
        ParseInstruction { pattern }
    }
}

impl Parse<Instruction> for ParseInstruction {
    fn parse_once(slice: &str) -> (Option<Instruction>, &str) {
        (None, slice)
    }
}

trait PatternPart<T> {
    fn parse(slice: &str) -> Option<(Vec<T>, &str)>;
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
