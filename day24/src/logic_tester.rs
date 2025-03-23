use crate::logic::*;

pub struct LogicTester {
    logic: Logic,
}

const TEST_CASES: [(usize, usize); _] = {
    let mut test_cases = Vec::new();
    test_cases.push((0, 0));
    test_cases.push((0, 1));
    test_cases.push((1, 0));
    test_cases.push((1, 1));
    for x in 0..4 {
        for y in 0..4 {
            for bit_index in 0..INPUT_BITS - 2 {
                test_cases.push((x << bit_index, y << bit_index));
            }
        }
    }

    test_cases
};
