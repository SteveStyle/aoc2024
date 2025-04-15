#![allow(dead_code)]
#![allow(unused_variables)]
use crate::logic::{
    wire_helpers::{GateDependencies, WireName},
    *,
};

const NO_CASES: usize = 1 + INPUT_BITS * 2;
pub const TEST_CASES: [(usize, usize); NO_CASES] = {
    const ALL_ONES: usize = (2 << INPUT_BITS) - 1;
    let mut test_cases = [(0, 0); 1 + INPUT_BITS * 2];
    let mut case_idx = 0;
    test_cases[case_idx] = (0, 0);
    case_idx += 1;
    let mut i = 0;
    while i < INPUT_BITS {
        test_cases[case_idx] = (0, 1 << i);
        case_idx += 1;
        test_cases[case_idx] = (1 << i, 3 << i);
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
pub struct LogicTester {
    logic: Logic<NO_CASES>,
}

impl LogicTester {
    pub fn new(input: &str) -> Self {
        let mut logic = Logic::<NO_CASES>::new_with_cases(input, TEST_CASES);
        Self { logic }
    }

    pub fn identify_swaps(&mut self) {
        let test_outputs_by_case = self.logic.eval_output();

        // print the test outputs.  x, y, actual and expected should be printed as integers.  z wire misses should be printed as binary.
        for i in 0..NO_CASES {
            let test_output = test_outputs_by_case[i];
            println!("Test case {:2}: {:?}", i, test_output);
        }

        let z_wire_any_miss = test_outputs_by_case
            .iter()
            .fold(0, |acc, &x| acc | x.z_wire_misses);

        // any gate which is a dependency for a good z wire should be marked as good
        let mut good_gates = GateDependencies::default();
        for i in 0..OUTPUT_BITS {
            if z_wire_any_miss & (1 << i) == 0 {
                // This bit is not set in z_wire_any_miss, so we can find the gates that are dependencies for this bit and mark them as good.
                let gate_dependencies = self.logic.engine[Z_OFFSET + i]
                    .wire_analytics
                    .gate_dependencies;
                good_gates |= gate_dependencies;
            }
        }

        for i in 0..OUTPUT_BITS {
            // if z wire is good then print the gate dependencies
            if z_wire_any_miss & (1 << i) == 0 {
                let gate_dependencies = self.logic.engine[Z_OFFSET + i]
                    .wire_analytics
                    .gate_dependencies;
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
        self.logic
            .engine
            .iter()
            .take_while(|x| x.wire_name < WireName::default())
            .filter(|x| good_gates.get(x.wire_index))
            .for_each(|x| print!(" {:#?}", x.wire_name.as_string()));
        println!();

        println!("Possibly bad gates: ");
        self.logic
            .engine
            .iter()
            .take_while(|x| x.wire_name < WireName::default())
            .filter(|x| !good_gates.get(x.wire_index))
            .for_each(|x| print!(" {:#?}", x.wire_name.as_string()));
        println!();
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
        let mut logic_tester = LogicTester::new(input);
        logic_tester.identify_swaps();
    }
}
