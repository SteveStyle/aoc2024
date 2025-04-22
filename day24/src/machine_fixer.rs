#![allow(dead_code)]
#![allow(unused_variables)]
use crate::machine::{
    wire_helpers::{GateArray, WireName},
    *,
};

const NO_CASES: usize = 1 + INPUT_BITS * 2;
pub const TEST_CASES: [InputPair; NO_CASES] = {
    const ALL_ONES: usize = (2 << INPUT_BITS) - 1;
    let mut test_cases = [InputPair { x: 0, y: 0 }; 1 + INPUT_BITS * 2];
    let mut case_idx = 0;
    test_cases[case_idx] = InputPair { x: 0, y: 0 };
    case_idx += 1;
    let mut i = 0;
    while i < INPUT_BITS {
        test_cases[case_idx] = InputPair { x: 0, y: 1 << i };
        case_idx += 1;
        test_cases[case_idx] = InputPair {
            x: 1 << i,
            y: 3 << i,
        };

        case_idx += 1;
        i += 1;
    }

    test_cases

    // [
    //     (0, 0),
    //     (0, 1),
    //     (1, 1),
    //     (ALL_ONES, 0),
    //     (1, ALL_ONES),
    //     (ALL_ONES, ALL_ONES),
    // ]
};

#[derive(Debug, Copy, Clone)]
pub struct MachineFixer {
    machine: Machine<NO_CASES>,
}

impl MachineFixer {
    pub fn new(input: &str) -> Self {
        let mut machine = Machine::<NO_CASES>::new_with_cases(input, TEST_CASES);
        Self { machine }
    }

    pub fn run_and_analyse(&mut self) {
        let test_outputs_by_case = self.machine.eval_output();

        // check that the gate analytics are valid
        for wire in self.machine.wires.iter() {
            match wire.wire_analytics.validate() {
                Ok(_) => {}
                Err(e) => {
                    println!("Wire {}: {}", wire.wire_name.as_string(), e);
                }
            }
        }

        // print the test outputs.  x, y, actual and expected should be printed as integers.  z wire misses should be printed as binary.
        for (i, test_output) in test_outputs_by_case.iter().enumerate() {
            println!("Test case {:2}: {:?}", i, test_output);
        }

        let z_wire_any_miss = test_outputs_by_case
            .iter()
            .fold(0, |acc, &x| acc | x.z_wire_misses);

        // any gate which is a dependency for a good z wire should be marked as good
        let mut good_gates = GateArray::default();
        for i in 0..OUTPUT_BITS {
            if z_wire_any_miss & (1 << i) == 0 {
                // This bit is not set in z_wire_any_miss, so we can find the gates that are dependencies for this bit and mark them as good.
                let gate_dependencies = self.machine.wires[Z_OFFSET + i].wire_analytics.gate_array;
                good_gates |= gate_dependencies;
            }
        }

        for i in 0..OUTPUT_BITS {
            // if z wire is good then print the gate dependencies
            if z_wire_any_miss & (1 << i) == 0 {
                let gate_dependencies = self.machine.wires[Z_OFFSET + i].wire_analytics.gate_array;
                let good_gate_list: Vec<String> = gate_dependencies
                    .0
                    .iter()
                    .map(|x| format!("{:#?}", x))
                    .collect();
                println!(
                    "Good gate dependencies for z{}: {}",
                    i,
                    good_gate_list.join(", ")
                );
            }
        }

        println!("Good gates: ");
        self.machine
            .wires
            .iter()
            .take_while(|x| x.wire_name < WireName::default())
            .filter(|x| good_gates.get(x.wire_index))
            .for_each(|x| print!(" {:#?}", x.wire_name.as_string()));
        println!();

        println!("Possibly bad gates: ");
        self.machine
            .wires
            .iter()
            .take_while(|x| x.wire_name < WireName::default())
            .filter(|x| !good_gates.get(x.wire_index))
            .for_each(|x| print!(" {:#?}", x.wire_name.as_string()));
        println!();

        for wire_idx in 0..NO_WIRES {
            if let Err(e) = self.machine.validate_wire(wire_idx) {
                println!("error on wire {}, {}", wire_idx, e);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::INPUT;
    use stephen_morris_utils::timer::time;

    #[test]
    fn test_logic_tester() {
        let input = INPUT;
        let mut machine_fixer = MachineFixer::new(input);
        machine_fixer.run_and_analyse();
    }
}
