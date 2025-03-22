#![allow(dead_code)]
use std::fmt::Debug;

use wire_helpers::{WireAnalytics, WireName, WireValue};

const INPUT_BITS: usize = 45;
const OUTPUT_BITS: usize = INPUT_BITS + 1;
const NO_GATES: usize = 313 - 91;
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
pub struct Logic {
    x: usize,
    y: usize,
    gates: [WireNameValue; NO_GATES],
    highest_z_bit: usize,
    engine: [EngineWire; NO_GATES + 2 * INPUT_BITS + OUTPUT_BITS],
}

impl Logic {
    pub fn new(input: &str) -> Self {
        println!("start of new");
        let mut x = 0;
        let mut y = 0;

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

        let mut gates = [WireNameValue::default(); NO_GATES];
        let mut idx = 0;
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

            gates[idx] = WireNameValue {
                wire_name: output,
                wire_value: WireValue::Connection {
                    input1,
                    input2,
                    operation,
                },
            };

            idx += 1;
        }
        gates.sort();

        println!("done part 1");

        let mut engine = [EngineWire::default(); NO_GATES + 2 * INPUT_BITS + OUTPUT_BITS];
        for &WireNameValue {
            wire_name,
            wire_value,
        } in &gates
        {
            let engine_idx = Self::get_gate_index(&gates, wire_name);
            let value_start = match wire_value {
                WireValue::Value(b) => WireValue::Value(b),
                WireValue::Connection {
                    input1,
                    input2,
                    operation,
                } => WireValue::Connection {
                    input1: Self::get_gate_index(&gates, input1),
                    input2: Self::get_gate_index(&gates, input2),
                    operation,
                },
            };
            engine[engine_idx] = EngineWire {
                wire_name,
                value_start,
                value_calc: value_start,
                wire_analytics: WireAnalytics::default(),
            };
        }

        for bit in 0..INPUT_BITS {
            let wire_name = WireName::from_char_bit(b'x', bit);
            engine[Self::get_gate_index(&gates, wire_name)] = EngineWire {
                wire_name,
                ..Default::default()
            };
            let wire_name = WireName::from_char_bit(b'y', bit);
            engine[Self::get_gate_index(&gates, wire_name)] = EngineWire {
                wire_name,
                ..Default::default()
            };
        }

        // z wires do not have wirename set, but pick up the default value Value(false)

        engine.sort();

        for gate in 0..engine.len() {
            print!("gate {} {:?}   ", gate, engine[gate].wire_name);
        }

        Self {
            x,
            y,
            gates,
            highest_z_bit,
            engine,
        }
    }

    // use a binary search to find the gate index in the sorted gates vector
    fn get_gate_index(gates: &[WireNameValue], wire_name: WireName) -> usize {
        match wire_name[0] {
            b'x' => NO_GATES + wire_name.bit().unwrap(),
            b'y' => NO_GATES + INPUT_BITS + wire_name.bit().unwrap(),
            b'z' => NO_GATES + INPUT_BITS * 2 + wire_name.bit().unwrap(),
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
        for eng in &mut self.engine {
            match eng.wire_name[0] {
                b'x' => {
                    eng.value_start =
                        WireValue::Value((x & (1 << eng.wire_name.bit().unwrap())) != 0)
                }
                b'y' => {
                    eng.value_start =
                        WireValue::Value((y & (1 << eng.wire_name.bit().unwrap())) != 0)
                }
                _ => {}
            }
            eng.value_calc = eng.value_start;
            eng.wire_analytics = WireAnalytics::default();
        }

        // eval the z gates, and collect as an integer using the bit value to detemine the binary columns
        let mut z = 0;
        for bit in 0..=self.highest_z_bit {
            let wire_name = WireName::from_char_bit(b'z', bit);
            let idx = Self::get_gate_index(&self.gates, wire_name);
            let (value, _) = self.eval(idx);
            if value {
                z |= 1 << bit;
            }
        }
        z
    }

    fn eval(&mut self, wire_idx: usize) -> (bool, WireAnalytics) {
        let engine_wire = self.engine[wire_idx];
        let (new_value, new_analytics) = match engine_wire.value_calc {
            WireValue::Value(b) => (b, engine_wire.wire_analytics),
            WireValue::Connection {
                input1,
                input2,
                operation,
            } => {
                let (input1, wa1) = self.eval(input1);
                let (input2, wa2) = self.eval(input2);
                let value = match operation {
                    Operation::And => input1 & input2,
                    Operation::Or => input1 | input2,
                    Operation::Xor => input1 ^ input2,
                };
                let wire_analytics = wa1.merge(&wa2);
                (value, wire_analytics)
            }
        };
        self.engine[wire_idx].value_calc = WireValue::Value(new_value);
        self.engine[wire_idx].wire_analytics = new_analytics;
        (new_value, new_analytics)
    }

    pub fn eval_output(&mut self) -> usize {
        self.calc(self.x, self.y)
    }

    fn set_variable(&mut self, variable: usize, variable_char: u8) {
        for bit in 0..INPUT_BITS {
            let bit_value = (variable & (1 << bit)) != 0;
            let idx =
                Self::get_gate_index(&self.gates, WireName::from_char_bit(variable_char, bit));
            self.engine[idx].value_calc = WireValue::Value(bit_value);
        }
    }

    fn get_variable(&mut self, variable_char: u8) -> usize {
        let mut variable = 0;

        for bit in 0..OUTPUT_BITS {
            let wire_name = WireName::from_char_bit(variable_char, bit);
            let idx = Self::get_gate_index(&self.gates, wire_name);
            let engine_wire = self.engine[idx];
            #[cfg(test)]
            if engine_wire.wire_name != wire_name {
                break;
            }
            match engine_wire.value_calc {
                WireValue::Value(b) => {
                    if b {
                        variable |= 1 << bit
                    }
                }
                WireValue::Connection { .. } => unreachable!(),
            }
        }
        variable
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_new() {
        let lm = Logic::new(TESTINPUT);
        println!("start of test_new");
        // for g in lm.gates {
        //     println!("{:?} {:?}", g.wire_name, g.wire_value);
        // }
    }
    #[test]
    fn test_part1() {
        let mut lm = Logic::new(TESTINPUT);
        assert_eq!(lm.calc(lm.x, lm.y), 4);
    }
    #[test]
    fn test_part1_2() {
        let mut lm = Logic::new(TESTINPUT2);
        assert_eq!(lm.calc(lm.x, lm.y), 2024);
    }

    // ...existing code...

    #[test]
    fn test_get_gate_index() {
        let lm = Logic::new(TESTINPUT2);

        println!("\ngate array");

        //test that each wire name in the array gives the correct index, and finds the same wire name
        // show the any wire name that does not index itself correctly. Show both the index and the wire name.
        // ignore wire names starting with 'w'.
        // print the wire name and the index if they do not match.
        for (idx, wire_name) in lm.gates.iter().enumerate() {
            if wire_name.wire_name[0] != b'w' {
                let found_idx = Logic::get_gate_index(&lm.gates, wire_name.wire_name);
                if found_idx != idx {
                    println!(
                        "wire name {:?} index {} found index {}",
                        wire_name.wire_name, idx, found_idx
                    );
                }
            }
        }

        println!("engine array");

        // run the same test for the engine array
        for (idx, engine_wire) in lm.engine.iter().enumerate() {
            if engine_wire.wire_name[0] != b'w' {
                let found_idx = Logic::get_gate_index(&lm.gates, engine_wire.wire_name);
                if found_idx != idx {
                    println!(
                        "wire name {:?} index {} found index {}",
                        engine_wire.wire_name, idx, found_idx
                    );
                }
            }
        }

        // Test x wire indices
        let x0 = WireName::from_char_bit(b'x', 0);
        assert_eq!(Logic::get_gate_index(&lm.gates, x0), NO_GATES);

        let x1 = WireName::from_char_bit(b'x', 1);
        assert_eq!(Logic::get_gate_index(&lm.gates, x1), NO_GATES + 1);

        // Test y wire indices
        let y0 = WireName::from_char_bit(b'y', 0);
        assert_eq!(Logic::get_gate_index(&lm.gates, y0), NO_GATES + INPUT_BITS);

        let y1 = WireName::from_char_bit(b'y', 1);
        assert_eq!(
            Logic::get_gate_index(&lm.gates, y1),
            NO_GATES + INPUT_BITS + 1
        );

        // Test gate wire indices (these should use binary search)
        // Note: Exact indices will depend on your test input gates
        let gate_wire = lm.gates[0].wire_name;
        assert_eq!(Logic::get_gate_index(&lm.gates, gate_wire), 0);
    }

    #[test]
    fn test_engine_initialization() {
        let lm = Logic::new(TESTINPUT);

        // Check x wire initialization
        let x0_idx = Logic::get_gate_index(&lm.gates, WireName::from_char_bit(b'x', 0));
        assert_eq!(lm.engine[x0_idx].wire_name[0], b'x');

        // Check y wire initialization
        let y0_idx = Logic::get_gate_index(&lm.gates, WireName::from_char_bit(b'y', 0));
        assert_eq!(lm.engine[y0_idx].wire_name[0], b'y');
    }
}
