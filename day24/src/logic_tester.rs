#![allow(dead_code)]
#![allow(unused_variables)]
use crate::logic::*;

const NO_CASES: usize = 4;
pub const TEST_CASES: [(usize, usize); NO_CASES] = {
    const ALL_ONES: usize = (2 << INPUT_BITS) - 1;
    [
        (0, 0),
        (0, 1),
        //(1, 1),
        (ALL_ONES, 0),
        (1, ALL_ONES),
        //        (ALL_ONES, ALL_ONES),
    ]
};

pub struct LogicTester {
    logic: Logic<NO_CASES>,
}

impl LogicTester {
    pub fn new(input: &str) -> Self {
        let mut logic = Logic::<NO_CASES>::new_with_cases(input, TEST_CASES);
        Self { logic }
    }

    pub fn test_and_show_wires(&mut self) {
        let output_array = self.logic.eval_output();
        for ((input1, input2), output) in TEST_CASES.iter().zip(output_array.iter()) {
            let expected = *input1 + *input2;
            // let output = output;
            let misses = output ^ expected;
            if misses != 0 {
                println!(
                    "Test failed: input1 = {:0width$b}, input2 = {:0width$b}, expected = {:0width$b}, output = {:0width$b}, misses = {:0width$b}",
                    input1,
                    input2,
                    expected,
                    output,
                    misses,
                    width = INPUT_BITS
                );
                for i in 0..INPUT_BITS {
                    if (misses & (1 << i)) != 0 {
                        print!("z{:02}, ", i);
                    }
                }
                println!();
                println!();
            }
        }
    }
    pub fn test_only(&mut self) {
        for ((input1, input2), output) in TEST_CASES.iter().zip(self.logic.eval_output().iter()) {
            let expected = *input1 + *input2;
            let misses = output ^ expected;
        }
    }
    pub fn test(&mut self) -> usize {
        let mut all_misses = 0;
        for ((input1, input2), output) in TEST_CASES.iter().zip(self.logic.eval_output().iter()) {
            let expected = *input1 + *input2;
            let misses = output ^ expected;
            all_misses |= misses;
        }
        all_misses
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::INPUT;
    use stephen_morris_utils::timer::time;

    #[test]
    fn test_logic() {
        let mut tester = LogicTester::new(INPUT);
        let result = time(|| tester.test_and_show_wires(), "test_logic");
        result.print_duration();
    }
    #[test]
    fn test_only() {
        fn test(logic: &mut Logic<NO_CASES>, cases: &[(usize, usize)]) -> usize {
            let mut all_misses = 0;
            for ((input1, input2), output) in cases.iter().zip(logic.eval_output().iter()) {
                let expected = *input1 + *input2;
                let misses = output ^ expected;
                all_misses |= misses;
            }
            all_misses
        }
        let mut logic = time(
            || Logic::new_with_cases(INPUT, TEST_CASES),
            "Uninitialised Logic",
        );
        logic.print_duration();
        let cases = time(|| test(&mut logic, &TEST_CASES), "9 cases");
        cases.print_all();
        // let cases = time(|| test(&mut logic, &TEST_CASES_ONE), "1 case");
        // cases.print_all();
        // let cases = time(|| test(&mut logic, &TEST_CASES_MANY), "45 case");
        // cases.print_all();
    }
}
