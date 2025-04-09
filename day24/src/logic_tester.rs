#![allow(dead_code)]
#![allow(unused_variables)]
use crate::logic::{
    wire_helpers::{GateDependencies, WireName},
    *,
};

const NO_CASES: usize = 6;
pub const TEST_CASES: [(usize, usize); NO_CASES] = {
    const ALL_ONES: usize = (2 << INPUT_BITS) - 1;
    [
        (0, 0),
        (0, 1),
        (1, 1),
        (ALL_ONES, 0),
        (1, ALL_ONES),
        (ALL_ONES, ALL_ONES),
    ]
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
