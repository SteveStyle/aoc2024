use std::fmt::Debug;

use wire_helpers::{GateFlags, WireAnalytics, WireName, WireValue};

pub const INPUT_BITS: usize = 45;
pub const OUTPUT_BITS: usize = INPUT_BITS + 1;
pub const NO_GATES: usize = 313 - 91;
pub const X_OFFSET: usize = NO_GATES;
pub const Y_OFFSET: usize = NO_GATES + INPUT_BITS;
pub const Z_OFFSET: usize = NO_GATES + 2 * INPUT_BITS;
pub mod wire_helpers;

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
#[derive(Debug, Clone, Copy)]
pub struct Logic {
    x: usize,
    y: usize,
    gates: [WireNameValue; NO_GATES],
    highest_z_bit: usize,
    engine: [EngineWire; NO_GATES + 2 * INPUT_BITS + OUTPUT_BITS],
}

const DEFAULT_WIRE_NAME: WireName = WireName([0; 3]);
const DEFAULT_WIRE_VALUE: WireValue<usize> = WireValue::Value(false);
const DEFAULT_GATE_FLAGS: GateFlags = GateFlags([0, 2]);
const DEFAULT_WIRE_ANALYTICS: WireAnalytics = WireAnalytics {
    gates: DEFAULT_GATE_FLAGS,
};
const DEFAULT_WIRE_NAME_VALUE: WireNameValue = WireNameValue {
    wire_name: DEFAULT_WIRE_NAME,
    wire_value: WireValue::Value(false),
};
const DEFAULT_ENGINE_WIRE: EngineWire = EngineWire {
    wire_name: DEFAULT_WIRE_NAME,
    value_start: DEFAULT_WIRE_VALUE,
    value_calc: DEFAULT_WIRE_VALUE,
    wire_analytics: DEFAULT_WIRE_ANALYTICS,
};
pub const DEFAULT_LOGIC: Logic = Logic {
    x: 0,
    y: 0,
    gates: [DEFAULT_WIRE_NAME_VALUE; NO_GATES],
    highest_z_bit: 0,
    engine: [DEFAULT_ENGINE_WIRE; NO_GATES + 2 * INPUT_BITS + OUTPUT_BITS],
};

impl Logic {
    pub fn new_uninitialised() -> Self {
        DEFAULT_LOGIC
    }
    pub fn initialise(&mut self, input: &str) -> &mut Self {
        let mut x = 0;
        let mut y = 0;

        let (input_values, input_gates) = input.split_once("\n\n").unwrap();

        for line in input_values.lines() {
            let bytes = line.as_bytes();
            let bit_index = WireName::from_slice(&bytes[0..3]).bit_index().unwrap();
            let bit_value = bytes[5] == b'1';
            if bit_value {
                if bytes[0] == b'x' {
                    x |= 1 << bit_index
                } else {
                    y |= 1 << bit_index
                }
            }
        }

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

        for bit in highest_z_bit + 1..OUTPUT_BITS {
            let wire_name = WireName::from_char_bit(b'z', bit);
            engine[Self::get_gate_index(&gates, wire_name)] = EngineWire {
                wire_name,
                ..Default::default()
            };
        }

        engine.sort();

        *self = Self {
            x,
            y,
            gates,
            highest_z_bit,
            engine,
        };
        self
    }

    // use a binary search to find the gate index in the sorted gates vector
    fn get_gate_index(gates: &[WireNameValue], wire_name: WireName) -> usize {
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
        let gate_flags = self.engine[gate_idx].wire_analytics.gates;
        let mut gates_list = Vec::new();
        for (idx, engine_wire) in self.engine.iter().enumerate() {
            if gate_flags.get(idx) {
                gates_list.push(engine_wire.wire_name);
            }
        }
        gates_list
    }

    pub fn calc(&mut self, x: usize, y: usize) -> usize {
        for bit in 0..INPUT_BITS {
            let x_wire_name = WireName::from_char_bit(b'x', bit);
            let y_wire_name = WireName::from_char_bit(b'y', bit);
            let x_idx = Self::get_gate_index(&self.gates, x_wire_name);
            let y_idx = Self::get_gate_index(&self.gates, y_wire_name);
            self.engine[x_idx].value_start = WireValue::Value((x & (1 << bit)) != 0);
            self.engine[y_idx].value_start = WireValue::Value((y & (1 << bit)) != 0);
        }
        for engine_wire in &mut self.engine {
            engine_wire.value_calc = engine_wire.value_start;
            engine_wire.wire_analytics = WireAnalytics::default();
        }

        // eval the z gates, and collect as an integer using the bit value to detemine the binary columns
        let mut z = 0;
        for bit_index in 0..=self.highest_z_bit {
            let wire_name = WireName::from_char_bit(b'z', bit_index);
            let idx = Self::get_gate_index(&self.gates, wire_name);
            let (value, _) = self.eval(idx);
            if value {
                z |= 1 << bit_index;
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
                let mut wire_analytics = wa1.merge(&wa2);
                wire_analytics.gates.set(wire_idx);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn test_wire_analytics(input: &str) {
        let mut lm = DEFAULT_LOGIC;
        lm.initialise(input);
        lm.eval_output();
        lm.engine
            .iter()
            .filter(|ew| ew.wire_name[0] == b'z' && !ew.wire_analytics.gates.is_empty())
            .for_each(|engine_wire| {
                println!(
                    "{:?} {:?}",
                    engine_wire.wire_name,
                    engine_wire.wire_analytics.gates.as_binary_string()
                );
                // use get_gates to get the wire names for the gate indexes set in the gate flags
                let gates = lm.get_gates(Logic::get_gate_index(&lm.gates, engine_wire.wire_name));
                for gate in gates {
                    println!("  {:?}", gate);
                }
            });
    }

    #[test]
    fn test_new() {
        println!("\nTest new");
        test_wire_analytics(TESTINPUT2);
    }
    #[test]
    fn test_part1() {
        let mut lm = DEFAULT_LOGIC;
        lm.initialise(TESTINPUT);
        assert_eq!(lm.calc(lm.x, lm.y), 4);
    }
    #[test]
    fn test_part1_2() {
        let mut lm = DEFAULT_LOGIC;
        lm.initialise(TESTINPUT2);
        assert_eq!(lm.calc(lm.x, lm.y), 2024);
    }

    // ...existing code...

    #[test]
    fn test_get_gate_index() {
        let mut lm = DEFAULT_LOGIC;
        lm.initialise(TESTINPUT2);

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
        let mut lm = DEFAULT_LOGIC;
        lm.initialise(TESTINPUT);

        // Check x wire initialization
        let x0_idx = Logic::get_gate_index(&lm.gates, WireName::from_char_bit(b'x', 0));
        assert_eq!(lm.engine[x0_idx].wire_name[0], b'x');

        // Check y wire initialization
        let y0_idx = Logic::get_gate_index(&lm.gates, WireName::from_char_bit(b'y', 0));
        assert_eq!(lm.engine[y0_idx].wire_name[0], b'y');
    }
}
