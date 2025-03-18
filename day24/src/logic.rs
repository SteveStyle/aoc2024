#![allow(dead_code)]
use std::{collections::HashMap, fmt::Debug, ops::Deref};

use wire_helpers::{WireAnalytics, WireName, WireValue, WireValuePayload};

const INPUT_BITS: usize = 45;
const OUTPUT_BITS: usize = INPUT_BITS + 1;
const NO_GATES: usize = 313 - 91;
const HASHMAP_SIZE: usize = 2 * INPUT_BITS + NO_GATES;
mod wire_helpers;

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
pub enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct EngineWire {
    wire_name: WireName,
    value_start: WireValue<usize>,
    value_calc: WireValue<usize>,
    wire_analytics: WireAnalytics,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct WireNameValue {
    wire_name: WireName,
    wire_value: WireValue<WireName>,
}
#[derive(Debug)]
pub struct LogicMaster {
    x: usize,
    y: usize,
    gates: [WireName; NO_GATES],
    // gates: Vec<WireNameValue>,
    highest_z_bit: usize,
    engine: [EngineWire; NO_GATES + 2 * INPUT_BITS],
    // engine: Vec<EngineWire>,
}

impl LogicMaster {
    pub fn new(input: &str) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut gates_v = Vec::with_capacity(NO_GATES + OUTPUT_BITS);

        let (input_values, input_gates) = input.split_once("\n\n").unwrap();

        for line in input_values.lines() {
            let bytes = line.as_bytes();
            let level = WireName::from_slice(&bytes[0..3]).bit().unwrap();
            let bit_value = bytes[5] == b'1';
            if bit_value {
                if bytes[0] == b'x' {
                    x |= 1 << level
                } else {
                    y |= 1 << level
                }
            }
        }

        let mut highest_z_bit = 0;
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
            if output[0] == b'z' {
                highest_z_bit = highest_z_bit.max(output.bit().unwrap());
            }
            gates_v.push(WireNameValue {
                wire_name: output,
                wire_value: WireValue::Connection {
                    input1,
                    input2,
                    operation,
                },
            });
        }

        gates_v.sort();

        let gates: [WireNameValue; NO_GATES] = [Default::default(); NO_GATES];
        gates.copy_from_slice(&gates_v);

        let mut engine = [EngineWire::default(); NO_GATES + 2 * INPUT_BITS];
        for &WireNameValue {
            wire_name,
            wire_value,
        } in &gates_v
        {
            let engine_idx = Self::get_gate_index(&gates_v, wire_name);
            let value_start = match wire_value {
                WireValue::Value(b) => WireValue::Value(b),
                WireValue::Connection {
                    input1,
                    input2,
                    operation,
                } => WireValue::Connection {
                    input1: Self::get_gate_index(&gates_v, input1),
                    input2: Self::get_gate_index(&gates_v, input2),
                    operation,
                },
            };
            engine[engine_idx] = EngineWire {
                wire_name,
                value_start,
                value_calc: value_start.clone(),
                wire_analytics: WireAnalytics::default(),
            };
        }

        for bit in 0..INPUT_BITS {
            let wire_name = WireName::from_char_bit(b'x', bit as u8);
            engine[Self::get_gate_index(&gates_v, wire_name)] = EngineWire {
                wire_name,
                ..Default::default()
            };
            let wire_name = WireName::from_char_bit(b'y', bit as u8);
            engine[Self::get_gate_index(&gates_v, wire_name)] = EngineWire {
                wire_name,
                ..Default::default()
            };
        }

        engine.sort();

        Self {
            x,
            y,
            gates: gates_v,
            highest_z_bit,
            engine,
        }
    }

    // use a binary search to find the gate index in the sorted gates vector
    fn get_gate_index(gates: &Vec<WireNameValue>, wire_name: WireName) -> usize {
        match wire_name[0] {
            b'x' => NO_GATES + wire_name.bit().unwrap(),
            b'y' => NO_GATES + INPUT_BITS + wire_name.bit().unwrap(),
            // b'z' => NO_GATES + INPUT_BITS * 2 + wire_name.bit().unwrap(),
            _ => {
                let mut low = 0;
                let mut high = gates.len();
                while low < high {
                    let mid = (low + high) / 2;
                    match gates[mid].wire_name.cmp(&wire_name) {
                        std::cmp::Ordering::Less => low = mid + 1,
                        std::cmp::Ordering::Equal => return mid,
                        std::cmp::Ordering::Greater => high = mid,
                    }
                }
                low
            }
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

        debug_assert_eq!(self.working_logic.get_variable(b'x'), x);
        debug_assert_eq!(self.working_logic.get_variable(b'y'), y);

        let mut ear = Ear::default();
        for bit in (0..=self.highest_z_bit).rev() {
            let zname = WireName::from_char_bit(b'z', u8::try_from(bit).expect("bit fits in u8"));
            let wvp = self.working_logic.eval(zname);
            #[cfg(test)]
            println!("zname {zname:?} wvp {wvp:?}");
            ear.push(zname, wvp);
        }
        #[cfg(test)]
        println!("ear {ear:?}");

        ear.value

        // self.working_logic.eval_output().value
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
                let WireValuePayload(input1_value, input1_analytics) = self.eval(input1_value);
                let WireValuePayload(input2_value, input2_analytics) = self.eval(input2_value);
                let value = match operation {
                    Operation::And => input1_value & input2_value,
                    Operation::Or => input1_value | input2_value,
                    Operation::Xor => input1_value ^ input2_value,
                };
                let analytics = input1_analytics.merge(&input2_analytics);
                let wvp = WireValuePayload(value, analytics);
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
                WireValue::Value(WireValuePayload(bit_value, WireAnalytics::default())),
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
                Some(WireValue::Value(WireValuePayload(stored_bit_value, _))) => {
                    if *stored_bit_value {
                        variable |= 1 << bit
                    }
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
            WireValue::Value(WireValuePayload(stored_bit_value, _)) => {
                *variable += *stored_bit_value as usize
            }
            WireValue::Connection { .. } => unreachable!(),
        }
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
