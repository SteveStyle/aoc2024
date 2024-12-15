trait Parser<'a> : Iterator {
    fn new(slice: &str) -> Self;

    fn get_slice() -> &'a str;
    fn set_slice(slice: &str);

    fn parse() -> Option<(Self::Item, &'a str)>;

    fn next(&mut self) -> Option<Self::Item> {
        let (item, slice) = Self::parse()?;
        Self::set_slice(slice);
        Some(item)
    }

    fn parse_all() -> (Vec<Self::Item>, &'a str) {
        let mut v = Vec::new();
        while let Some(value) = Self::next() {
            v.push(value);
        }
        (v, Self::get_slice())
    }
}


struct InstructionParser<'a> {
    data: &'a str,
}

impl<'a, T> Parser<'a, T> for InstructionParser<'a> {
    fn new(slice: &str) -> Self { 
        Self {data: slice}
     };
}

impl<T> Iterator for InstructionParser<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {}
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
    fn parse_value(slice: &mut &str) -> Option(Instruction) {}
}

pub fn parse_input2(input: &str) -> Vec<Instruction> {
    let ret = Vec::new();
    let mut i = 0;

    ret
}
