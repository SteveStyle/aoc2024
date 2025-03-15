#![allow(dead_code)]
use std::{collections::HashMap, fmt::Debug, ops::Deref};

use wire_helpers::{WireAnalytics, WireName, WireValue, WireValuePayload};

const INPUT_BITS: usize = 45;
const OUTPUT_BITS: usize = INPUT_BITS + 1;
const NO_GATES: usize = 313 - 91;
const HASHMAP_SIZE: usize = 2 * INPUT_BITS + NO_GATES;
mod wire_helpers;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, Default)]
pub struct LogicMaster {
    x: usize,
    y: usize,
    gates: Vec<(WireName, WireValue)>,
    working_logic: Logic,
}

impl LogicMaster {
    pub fn new(input: &str) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut gates = Vec::new();

        let (input_values, input_gates) = input.split_once("\n\n").unwrap();

        for line in input_values.lines() {
            let bytes = line.as_bytes();
            let level = WireName::from_slice(&bytes[0..3]).level().unwrap();
            if bytes[0] == b'X' {
                x |= 1 << level
            } else {
                y |= 1 << level
            };
        }

        for line in input_gates.lines() {
            let mut bytes = line.as_bytes();
            let input1 = WireName::from_slice(&bytes[0..3]);
            let operation = match &bytes[4..7] {
                b"AND" => Operation::And,
                b"XOR" => Operation::Xor,
                b"OR " => Operation::Or,
                _ => unreachable!(),
            };

            if operation == Operation::Or {
                bytes = &bytes[7..];
            } else {
                bytes = &bytes[8..];
            }
            let input2 = WireName::from_slice(&bytes[0..3]);
            let output = WireName::from_slice(&bytes[7..]);
            gates.push((
                output,
                WireValue::Connection {
                    input1,
                    input2,
                    operation,
                },
            ));
        }

        let working_logic = Logic::new();
        Self {
            x,
            y,
            gates,
            working_logic,
        }
    }
    pub fn calc(&mut self, x: usize, y: usize) -> usize {
        self.working_logic.wires.clear();
        //TODO: accept wire swaps
        for (name, connection) in self.gates.iter() {
            self.working_logic.wires.insert(*name, *connection);
        }
        self.working_logic.set_variable(x, b'x');
        self.working_logic.set_variable(y, b'y');
        self.working_logic.eval_output().value
    }
    pub fn eval_output(&mut self) -> usize {
        self.calc(self.x, self.y)
    }

    const CASES: [[usize; 2]; 8] = [
        [0, 0],
        [0, 2],
        [2, 0],
        [2, 2],
        [1, 1],
        [1, 3],
        [3, 1],
        [3, 3],
    ];
    fn test_bits(&mut self) {
        for level in 0..INPUT_BITS {
            for case in Self::CASES {
                let [mut x, mut y] = case;
                x <<= level;
                y <<= level;
                let z = x + y;
                let result = self.calc(x, y);
                if result != z {
                    println!(
                        "test bit failed.  n {level:2}  x {x:20} y {y:20} z {z:20} result {result:14}  case {case:?} "
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Logic {
    wires: HashMap<WireName, WireValue>,
}

impl Logic {
    pub fn new() -> Self {
        Logic {
            wires: HashMap::with_capacity(HASHMAP_SIZE),
        }
    }

    fn eval(&mut self, wire_name: WireName) -> WireValuePayload {
        let wire_value = self.wires.get(&wire_name).unwrap().to_owned();
        match wire_value {
            WireValue::Value(wvp) => wvp,
            WireValue::Connection {
                input1: input1_value,
                input2: input2_value,
                operation,
            } => {
                let (input1_value, input1_analytics) = self.eval(input1_value);
                let (input2_value, input2_analytics) = self.eval(input2_value);
                let value = match operation {
                    Operation::And => input1_value & input2_value,
                    Operation::Or => input1_value | input2_value,
                    Operation::Xor => input1_value ^ input2_value,
                };
                let analytics = input1_analytics.merge(&input2_analytics);
                let wvp = (value, analytics);
                self.wires.insert(wire_name, WireValue::Value(wvp));
                wvp
            }
        }
    }

    fn set_variable(&mut self, variable: usize, variable_char: u8) {
        for bit in 0..INPUT_BITS {
            let bit_value = (variable & (1 << bit)) != 0;
            self.wires.insert(
                WireName::from_char_bit(variable_char, u8::try_from(bit).expect("bit fits in u8")),
                WireValue::Value((bit_value, WireAnalytics::default())),
            );
        }
    }

    fn get_variable(&mut self, variable_char: u8) -> usize {
        let mut variable = 0;
        for bit in 0..OUTPUT_BITS {
            match self.wires.get(&WireName::from_char_bit(
                variable_char,
                u8::try_from(bit).expect("bit fits in u8"),
            )) {
                Some(WireValue::Value((stored_bit_value, _))) => {
                    variable += *stored_bit_value as usize
                }
                None => {}
                Some(WireValue::Connection { .. }) => unreachable!(),
            }
        }
        variable
    }

    fn get_bit_value(&self, variable: &mut usize, input_char: u8, bit: usize) {
        match self
            .wires
            .get(&WireName::from_char_bit(
                input_char,
                u8::try_from(bit).expect("bit fits in u8"),
            ))
            .unwrap()
        {
            WireValue::Value((stored_bit_value, _)) => *variable += *stored_bit_value as usize,
            WireValue::Connection { .. } => unreachable!(),
        }
    }

    pub fn eval_output(&mut self) -> Ear {
        let mut ear = Ear::default();
        for bit in 0..OUTPUT_BITS {
            let zname = WireName::from_char_bit(b'z', u8::try_from(bit).expect("bit fits in u8"));
            let wvp = self.eval(zname);
            ear.push(zname, wvp);
        }
        ear
    }
}

//Eval All Response type
#[derive(Debug, Default)]
pub struct Ear {
    value: usize,
    values: Vec<(WireName, WireValuePayload)>,
}

impl Deref for Ear {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ear {
    fn push(&mut self, name: WireName, wvp: WireValuePayload) {
        self.value <<= 1;
        self.value += wvp.0 as usize;
        self.values.push((name, wvp));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_new() {
        let lm = LogicMaster::new(TESTINPUT);
        println!("{lm:#?}");
        assert_eq!(lm.gates.len(), 3);
    }
    #[test]
    fn test_part1() {
        let mut lm = LogicMaster::new(TESTINPUT);
        assert_eq!(lm.calc(lm.x, lm.y), 4);
    }
    #[test]
    fn test_part1_2() {
        let mut lm = LogicMaster::new(TESTINPUT2);
        assert_eq!(lm.calc(lm.x, lm.y), 2024);
    }
}
