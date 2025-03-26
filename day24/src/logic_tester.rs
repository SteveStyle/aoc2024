use crate::logic::*;

pub struct LogicTester {
    logic: Logic,
}

const TEST_CASES: [(usize, usize); 9] = {
    const ALL_ONES: usize = (2 << INPUT_BITS) - 1;
    [
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (ALL_ONES, 0),
        (0, ALL_ONES),
        (ALL_ONES, 1),
        (1, ALL_ONES),
        (ALL_ONES, ALL_ONES),
    ]
};

const TEST_CASES_ONE: [(usize, usize); 1] = {
    const ALL_ONES: usize = (2 << INPUT_BITS) - 1;
    [(ALL_ONES, 1)]
};

const TEST_CASES_MANY: [(usize, usize); 45] = {
    const ALL_ONES: usize = (2 << INPUT_BITS) - 1;
    [
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (ALL_ONES, 0),
        (0, ALL_ONES),
        (ALL_ONES, 1),
        (1, ALL_ONES),
        (ALL_ONES, ALL_ONES),
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (ALL_ONES, 0),
        (0, ALL_ONES),
        (ALL_ONES, 1),
        (1, ALL_ONES),
        (ALL_ONES, ALL_ONES),
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (ALL_ONES, 0),
        (0, ALL_ONES),
        (ALL_ONES, 1),
        (1, ALL_ONES),
        (ALL_ONES, ALL_ONES),
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (ALL_ONES, 0),
        (0, ALL_ONES),
        (ALL_ONES, 1),
        (1, ALL_ONES),
        (ALL_ONES, ALL_ONES),
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (ALL_ONES, 0),
        (0, ALL_ONES),
        (ALL_ONES, 1),
        (1, ALL_ONES),
        (ALL_ONES, ALL_ONES),
    ]
};

impl LogicTester {
    pub fn new(input: &str) -> Self {
        let mut logic = DEFAULT_LOGIC;
        logic.initialise(input);
        Self { logic }
    }

    pub fn test_and_show_wires(&mut self) {
        for (input1, input2) in TEST_CASES.iter() {
            let expected = *input1 + *input2;
            let output = self.logic.calc(*input1, *input2);
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
        for (input1, input2) in TEST_CASES.iter() {
            let expected = *input1 + *input2;
            let output = self.logic.calc(*input1, *input2);
            let misses = output ^ expected;
        }
    }
    pub fn test(&mut self) {
        let mut all_misses = 0;
        for (input1, input2) in TEST_CASES.iter() {
            let expected = *input1 + *input2;
            let output = self.logic.calc(*input1, *input2);
            let misses = output ^ expected;
            all_misses |= misses;
        }
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
        fn test(logic: &mut Logic, cases: &[(usize, usize)]) -> usize {
            let mut all_misses = 0;
            for (input1, input2) in cases.iter() {
                let mut local_logic = *logic;
                let expected = *input1 + *input2;
                let output = local_logic.calc(*input1, *input2);
                let misses = output ^ expected;
                all_misses |= misses;
            }
            // println!("all_misses = {:0width$b}", all_misses, width = INPUT_BITS);
            all_misses
        }
        let mut logic = time(Logic::new_uninitialised, "Uninitialised Logic");
        logic.print_duration();
        let mut logic = time(|| logic.initialise(INPUT), "Initialised Logic");
        logic.print_duration();
        let cases = time(|| test(&mut logic, &TEST_CASES), "9 cases");
        cases.print_all();
        let cases = time(|| test(&mut logic, &TEST_CASES_ONE), "1 case");
        cases.print_all();
        let cases = time(|| test(&mut logic, &TEST_CASES_MANY), "45 case");
        cases.print_all();
    }
    #[test]
    fn test_array() {
        fn test(logic: &mut Logic, case: &(usize, usize)) -> usize {
            let (input1, input2) = case;
            let expected = *input1 + *input2;
            let output = logic.calc(*input1, *input2);
            let misses = output ^ expected;
            misses
        }
        fn test_array(input: &str, cases: &[(usize, usize)]) {
            let mut all_misses = 0;
            let mut logic = Logic::new_uninitialised();
            logic.initialise(input);
            let mut logic_array = [DEFAULT_LOGIC; 9];
            for i in 0..9 {
                logic_array[i] = logic;
                test(&mut logic_array[i], &cases[i]);
            }
        }
        let result = time(|| test_array(INPUT, &TEST_CASES), "test_array");
        result.print_duration();
    }
}
