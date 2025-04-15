#![allow(dead_code)]
use std::fmt::Debug;

use wire_helpers::{GateDependencies, WireAnalytics, WireName, WireValue};

use crate::bit_array::BitArray;

pub const INPUT_BITS: usize = 45;
pub const OUTPUT_BITS: usize = INPUT_BITS + 1;
pub const NO_GATES: usize = 313 - 91;
pub const X_OFFSET: usize = NO_GATES;
pub const Y_OFFSET: usize = NO_GATES + INPUT_BITS;
pub const Z_OFFSET: usize = NO_GATES + INPUT_BITS * 2;
pub const NO_WIRES: usize = NO_GATES + INPUT_BITS * 2 + OUTPUT_BITS;
pub mod wire_helpers;

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
pub enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EngineWire<const N: usize> {
    pub wire_name: WireName,
    pub wire_index: usize,
    value_start: WireValue<usize, N>,
    value_calc: WireValue<usize, N>,
    pub wire_analytics: WireAnalytics,
}

impl<const N: usize> PartialOrd for EngineWire<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        debug_assert!(!(self.depends_on(other) && other.depends_on(self)));
        if self.depends_on(other) {
            Some(std::cmp::Ordering::Greater)
        } else if other.depends_on(self) {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl<const N: usize> EngineWire<N> {
    pub fn depends_on(&self, other: &Self) -> bool {
        self.wire_analytics.gate_dependencies.get(other.wire_index)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct WireNameValue<const N: usize> {
    wire_name: WireName,
    wire_value: WireValue<WireName, N>,
}
#[derive(Debug, Clone, Copy)]
pub struct Logic<const NO_CASES: usize> {
    cases: [(usize, usize); NO_CASES],
    gates: [WireNameValue<NO_CASES>; NO_GATES],
    highest_z_bit: usize,
    pub engine: [EngineWire<NO_CASES>; NO_WIRES],
}

impl Logic<1> {
    pub fn new(input: &str) -> Self {
        let (input_values, input_gates) = input.split_once("\n\n").unwrap();

        let mut given_x = 0;
        let mut given_y = 0;
        for line in input_values.lines() {
            let bytes = line.as_bytes();
            let bit_index = WireName::from_slice(&bytes[0..3]).bit_index().unwrap();
            let bit_value = bytes[5] == b'1';
            if bit_value {
                if bytes[0] == b'x' {
                    given_x |= 1 << bit_index
                } else {
                    given_y |= 1 << bit_index
                }
            }
        }

        let cases = [(given_x, given_y)];

        Self::new_with_cases(input, cases)
    }
    pub fn calc(input: &str, x: usize, y: usize) -> usize {
        let mut logic = Logic::new_with_cases(input, [(x, y)]);
        // eval the z gates, and collect as an integer using the bit value to detemine the binary columns
        let mut z = BitArray::new();
        for bit_index in 0..=logic.highest_z_bit {
            let wire_name = WireName::from_char_bit(b'z', bit_index);
            let wire_idx = Self::get_gate_index(&logic.gates, wire_name);
            let (z_nth_bit, _) = logic.eval(wire_idx);
            if z_nth_bit.get(bit_index) {
                z.set(bit_index);
            }
        }
        z.0 as usize
    }
}

impl<const NO_CASES: usize> Logic<NO_CASES> {
    pub fn new_with_cases(input: &str, cases: [(usize, usize); NO_CASES]) -> Self {
        let (_, input_gates) = input.split_once("\n\n").unwrap();

        let mut gates = [WireNameValue::default(); NO_GATES];

        let mut highest_z_bit = 0;
        for (idx, line) in input_gates.lines().enumerate() {
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
                highest_z_bit = highest_z_bit.max(output.bit_index().unwrap());
            }

            gates[idx] = WireNameValue {
                wire_name: output,
                wire_value: WireValue::Connection {
                    input1,
                    input2,
                    operation,
                },
            };
        }
        gates.sort();

        let mut engine = [EngineWire::default(); NO_WIRES];
        for &WireNameValue {
            wire_name,
            wire_value,
        } in &gates
        {
            let wire_index = Self::get_gate_index(&gates, wire_name);
            let value_start = match wire_value {
                WireValue::Value(b) => continue, // for the dummy entries that pad the array
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
            engine[wire_index] = EngineWire {
                wire_name,
                wire_index,
                value_start,
                value_calc: value_start,
                wire_analytics: WireAnalytics::default(),
            };
        }

        for bit in 0..INPUT_BITS {
            let wire_name = WireName::from_char_bit(b'x', bit);
            let mut bit_indexed_by_cases = BitArray::new();
            // we want a u64 array of 0s and 1s for each bit up to INPUT_BITS.
            // the bit index in the u64 is the case number
            for (case_idx, case) in cases.iter().enumerate() {
                bit_indexed_by_cases.set_value(case_idx, (case.0 & (1 << bit)) != 0);
            }
            engine[X_OFFSET + bit] = EngineWire {
                wire_name,
                wire_index: X_OFFSET + bit,
                value_start: WireValue::Value(bit_indexed_by_cases),
                value_calc: WireValue::Value(bit_indexed_by_cases),
                wire_analytics: WireAnalytics::default(),
            };
            let wire_name = WireName::from_char_bit(b'y', bit);
            let mut bit_by_cases_array = BitArray::new();
            // we want a u64 array of 0s and 1s for each bit up to INPUT_BITS.
            // the bit index in the u64 is the case number
            for (case_idx, case) in cases.iter().enumerate() {
                bit_by_cases_array.set_value(case_idx, (case.1 & (1 << bit)) != 0);
            }
            engine[Y_OFFSET + bit] = EngineWire {
                wire_name,
                wire_index: Y_OFFSET + bit,
                value_start: WireValue::Value(bit_by_cases_array),
                value_calc: WireValue::Value(bit_by_cases_array),
                wire_analytics: WireAnalytics::default(),
            };
        }

        for bit in highest_z_bit + 1..OUTPUT_BITS {
            let wire_name = WireName::from_char_bit(b'z', bit);
            engine[Self::get_gate_index(&gates, wire_name)] = EngineWire {
                wire_name,
                ..Default::default()
            };
        }

        // engine.sort();

        Self {
            cases,
            gates,
            highest_z_bit,
            engine,
        }
    }

    // use a binary search to find the gate index in the sorted gates vector
    fn get_gate_index(gates: &[WireNameValue<NO_CASES>], wire_name: WireName) -> usize {
        match wire_name[0] {
            b'x' => X_OFFSET + wire_name.bit_index().unwrap(),
            b'y' => Y_OFFSET + wire_name.bit_index().unwrap(),
            b'z' => Z_OFFSET + wire_name.bit_index().unwrap(),
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

    // get a list of wirenames for the indexes set in the gate flags
    fn get_gates(&self, gate_idx: usize) -> Vec<WireName> {
        let gate_flags = self.engine[gate_idx].wire_analytics.gate_dependencies;
        let mut gates_list = Vec::new();
        for (idx, engine_wire) in self.engine.iter().enumerate() {
            if gate_flags.get(idx) {
                gates_list.push(engine_wire.wire_name);
            }
        }
        gates_list
    }

    fn eval(&mut self, wire_idx: usize) -> (BitArray, WireAnalytics) {
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

                let output = match operation {
                    Operation::And => input1 & input2,
                    Operation::Or => input1 | input2,
                    Operation::Xor => input1 ^ input2,
                };
                let mut wire_analytics = wa1.merge(&wa2);
                wire_analytics.gate_dependencies.set(wire_idx);
                (output, wire_analytics)
            }
        };
        self.engine[wire_idx].value_calc = WireValue::Value(new_value);

        self.engine[wire_idx].wire_analytics = new_analytics;
        (new_value, new_analytics)
    }

    #[inline(always)]
    pub fn eval_output(&mut self) -> [TestCaseOutput; NO_CASES] {
        let mut actual_by_case = [0_usize; NO_CASES];
        for bit_index in 0..=self.highest_z_bit {
            let z_wire_name = WireName::from_char_bit(b'z', bit_index);
            let z_wire_idx = Self::get_gate_index(&self.gates, z_wire_name);
            let (z_wire_value_by_case, _) = self.eval(z_wire_idx);
            for (case, answer) in actual_by_case.iter_mut().enumerate() {
                *answer |= (z_wire_value_by_case.get(case) as usize) << bit_index;
            }
        }
        let mut test_outputs_by_case = [TestCaseOutput::default(); NO_CASES];
        for (case_idx, (input1, input2)) in self.cases.iter().enumerate() {
            let expected = *input1 + *input2;
            let z_wire_misses = actual_by_case[case_idx] ^ expected;
            test_outputs_by_case[case_idx] = TestCaseOutput {
                x: *input1,
                y: *input2,
                actual: actual_by_case[case_idx],
                expected,
                z_wire_misses,
            };
        }
        test_outputs_by_case
    }
}

#[derive(Clone, Copy, Default)]
pub struct TestCaseOutput {
    pub x: usize,
    pub y: usize,
    pub actual: usize,
    pub expected: usize,
    pub z_wire_misses: usize,
}

impl Debug for TestCaseOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {:14}, y: {:14}, actual: {:15}, expected: {:15}, z_wire_misses: {:047b}",
            self.x, self.y, self.actual, self.expected, self.z_wire_misses
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn test_wire_analytics(input: &str) {
        let mut lm = Logic::<1>::new(input);
        lm.eval_output();
        lm.engine
            .iter()
            .filter(|ew| ew.wire_name[0] == b'z' && !ew.wire_analytics.gate_dependencies.is_empty())
            .for_each(|engine_wire| {
                println!(
                    "test_wire_analytics::{:?} {:?}",
                    engine_wire.wire_name,
                    engine_wire
                        .wire_analytics
                        .gate_dependencies
                        .as_binary_string()
                );
                // use get_gates to get the wire names for the gate indexes set in the gate flags
                let gates = lm.get_gates(Logic::get_gate_index(&lm.gates, engine_wire.wire_name));
                for gate in gates {
                    print!("  {:?}", gate);
                }
                println!();
            });
    }

    #[test]
    fn test_new() {
        println!("\ntest_new::starting");
        test_wire_analytics(TESTINPUT2);
        println!("\ntest_new::ending");
    }
    #[test]
    fn test_part1() {
        println!("\ntest_part1::starting");
        let mut lm = Logic::new(TESTINPUT);
        let test_outputs_by_case = lm.eval_output();
        assert_eq!(test_outputs_by_case[0].actual, 4);
        println!("\ntest_part1::ending");
    }
    #[test]
    fn test_part1_2() {
        println!("\ntest_part1_2::starting");
        let mut lm = Logic::new(TESTINPUT2);
        let test_outputs_by_case = lm.eval_output();
        assert_eq!(test_outputs_by_case[0].actual, 2024);
        println!("\ntest_part1_2::ending");
    }

    #[test]
    fn test_get_gate_index() {
        println!("\ntest_get_gate_index::starting");
        let mut lm = Logic::<1>::new(TESTINPUT2);

        println!("\ngate array");

        //test that each wire name in the array gives the correct index, and finds the same wire name
        // show the any wire name that does not index itself correctly. Show both the index and the wire name.
        // ignore wire names starting with 'w'.
        // print the wire name and the index if they do not match.
        for (
            idx,
            EngineWire {
                wire_name,
                wire_index,
                value_start,
                value_calc,
                wire_analytics,
            },
        ) in lm.engine.iter().enumerate()
        {
            if wire_name[0] != b'w' {
                let found_idx = Logic::get_gate_index(&lm.gates, *wire_name);
                if found_idx != idx {
                    println!(
                        "wire name {:?} index {} found index {}",
                        wire_name, idx, found_idx
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
        // Note: Exact indices will depend on the test input gates
        let gate_wire = lm.gates[0].wire_name;
        assert_eq!(Logic::get_gate_index(&lm.gates, gate_wire), 0);
        println!("test_get_gate_index::ending");
    }

    #[test]
    fn test_engine_initialization() {
        println!("\ntest_engine_initialization::starting");
        let mut lm = Logic::<1>::new(TESTINPUT);

        // Check x wire initialization
        let x0_idx = Logic::get_gate_index(&lm.gates, WireName::from_char_bit(b'x', 0));
        assert_eq!(lm.engine[x0_idx].wire_name[0], b'x');

        // Check y wire initialization
        let y0_idx = Logic::get_gate_index(&lm.gates, WireName::from_char_bit(b'y', 0));
        assert_eq!(lm.engine[y0_idx].wire_name[0], b'y');
        println!("test_engine_initialization::ending");
    }
}
