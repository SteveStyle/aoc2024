use itertools::Itertools;
use stephen_morris_utils::get_numbers;
type Register = u64;

#[derive(Debug)]
struct Program {
    instruction_pointer: usize,
    instructions: Vec<u8>,
}

impl Program {
    fn read_value(&mut self) -> Option<u8> {
        if self.instruction_pointer < self.instructions.len() {
            let value = Some(self.instructions[self.instruction_pointer]);
            self.instruction_pointer += 1;
            value
        } else {
            None
        }
    }
    fn read_instruction(&mut self) -> Option<(Operation, u8)> {
        if let (Some(operation), Some(operand)) =
            (self.read_value().map(Operation::from), self.read_value())
        {
            Some((operation, operand))
        } else {
            None
        }
    }
    fn dissasemble(&mut self) -> String {
        fn combo(v: u8) -> String {
            match v {
                0..=3 => v.to_string(),
                4 => "A".to_string(),
                5 => "B".to_string(),
                6 => "C".to_string(),
                _ => unreachable!(),
            }
        }
        let mut result = String::new();
        self.instruction_pointer = 0;
        while let Some((opcode, operand)) = self.read_instruction() {
            result.push_str(&format!("{:} {:} ", opcode as usize, operand));
            match opcode {
                Operation::Adv => {
                    result.push_str(&format!("A = A / (2^ {:})\n", combo(operand)));
                }
                Operation::Bxl => {
                    // self.state.register_b ^= operand as Register;
                    result.push_str(&format!("B = B XOR {:}\n", operand));
                }
                Operation::Bst => {
                    // self.state.register_b = self.state.combo(operand) % 8;
                    result.push_str(&format!("B = {:} MOD 8\n", combo(operand)));
                }
                Operation::Jnz => {
                    result.push_str(&format!("Jump if A != 0 to {:}\n", combo(operand)));
                }
                Operation::Bxc => {
                    // self.state.register_b ^= self.state.register_c;
                    result.push_str("B = B XOR C\n");
                }
                Operation::Out => {
                    // self.state.output.push(self.state.combo(operand) % 8);
                    result.push_str(&format!("Output {:} MOD 8\n", combo(operand)));
                }
                Operation::Bdv => {
                    // self.state.register_b = (self.state.register_a >> self.state.combo(operand));
                    result.push_str(&format!("B = A / (2^ {:})\n", combo(operand)));
                }
                Operation::Cdv => {
                    // self.state.register_c = (self.state.register_a >> self.state.combo(operand));
                    result.push_str(&format!("C = A / (2^ {:})\n", combo(operand)));
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct ComputerState {
    register_a: Register,
    register_b: Register,
    register_c: Register,
    output: Vec<u8>,
}

impl ComputerState {
    fn combo(&self, v: u8) -> Register {
        match v {
            0..=3 => v as Register,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Computer {
    state: ComputerState,
    program: Program,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Operation {
    fn from(v: u8) -> Self {
        match v {
            0 => Operation::Adv,
            1 => Operation::Bxl,
            2 => Operation::Bst,
            3 => Operation::Jnz,
            4 => Operation::Bxc,
            5 => Operation::Out,
            6 => Operation::Bdv,
            7 => Operation::Cdv,
            _ => unreachable!(),
        }
    }
}

impl Computer {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let register_a = get_numbers(lines.next().unwrap())[0];
        let register_b = get_numbers(lines.next().unwrap())[0];
        let register_c = get_numbers(lines.next().unwrap())[0];
        lines.next();
        let instructions = get_numbers(lines.next().unwrap());
        let state = ComputerState {
            register_a,
            register_b,
            register_c,
            output: Vec::with_capacity(instructions.len()),
        };
        let program = Program {
            instruction_pointer: 0,
            instructions,
        };
        Computer { state, program }
    }
    fn execute_next_instruction(&mut self) -> bool {
        if let Some((opcode, operand)) = self.program.read_instruction() {
            match opcode {
                Operation::Adv => {
                    self.state.register_a >>= self.state.combo(operand);
                }
                Operation::Bxl => {
                    self.state.register_b ^= operand as Register;
                }
                Operation::Bst => {
                    self.state.register_b = self.state.combo(operand) % 8;
                }
                Operation::Jnz => {
                    if self.state.register_a != 0 {
                        self.program.instruction_pointer = operand as usize
                    }
                }
                Operation::Bxc => {
                    self.state.register_b ^= self.state.register_c;
                }
                Operation::Out => {
                    self.state.output.push(self.state.combo(operand) as u8 % 8);
                }
                Operation::Bdv => {
                    self.state.register_b = (self.state.register_a >> self.state.combo(operand));
                }
                Operation::Cdv => {
                    self.state.register_c = (self.state.register_a >> self.state.combo(operand));
                }
            }
            true
        } else {
            false
        }
    }
    pub fn execute_program(&mut self) -> String {
        // println!("{:?}", self);
        while self.execute_next_instruction() {
            // println!("{:?}", self);
        }
        self.state.output.iter().join(",")
    }

    pub fn execute_program_and_check(&mut self) -> String {
        while let Some((opcode, operand)) = self.program.read_instruction() {
            match opcode {
                Operation::Adv => {
                    self.state.register_a >>= self.state.combo(operand);
                }
                Operation::Bxl => {
                    self.state.register_b ^= operand as Register;
                }
                Operation::Bst => {
                    self.state.register_b = self.state.combo(operand) % 8;
                }
                Operation::Jnz => {
                    if self.state.register_a != 0 {
                        self.program.instruction_pointer = operand as usize
                    }
                }
                Operation::Bxc => {
                    self.state.register_b ^= self.state.register_c;
                }
                Operation::Out => {
                    self.state.output.push(self.state.combo(operand) as u8 % 8);
                }
                Operation::Bdv => {
                    self.state.register_b = (self.state.register_a >> self.state.combo(operand));
                }
                Operation::Cdv => {
                    self.state.register_c = (self.state.register_a >> self.state.combo(operand));
                }
            }
        }
        self.state.output.iter().join(",")
    }
    fn reset(&mut self, new_a: Register) {
        self.state.register_a = new_a;
        self.state.register_b = 0;
        self.state.register_c = 0;
        self.state.output.clear();
        self.program.instruction_pointer = 0;
    }
    pub fn find_initial_a(&mut self) -> Register {
        let mut i = 1 << 54;
        'outer: loop {
            if i % (100000000) == 0 {
                println!("{}", i);
            }
            self.reset(i);
            let mut j = 0;
            while let Some((opcode, operand)) = self.program.read_instruction() {
                match opcode {
                    Operation::Adv => {
                        self.state.register_a >>= self.state.combo(operand);
                    }
                    Operation::Bxl => {
                        self.state.register_b ^= operand as Register;
                    }
                    Operation::Bst => {
                        self.state.register_b = self.state.combo(operand) % 8;
                    }
                    Operation::Jnz => {
                        if self.state.register_a != 0 {
                            self.program.instruction_pointer = operand as usize
                        }
                    }
                    Operation::Bxc => {
                        self.state.register_b ^= self.state.register_c;
                    }
                    Operation::Out => {
                        self.state.output.push(self.state.combo(operand) as u8 % 8);
                        if self.program.instructions[j] != self.state.combo(operand) as u8 % 8 {
                            i += 1;
                            j = 0;
                            continue 'outer;
                        } else {
                            j += 1;
                        }
                    }
                    Operation::Bdv => {
                        self.state.register_b =
                            (self.state.register_a >> self.state.combo(operand));
                    }
                    Operation::Cdv => {
                        self.state.register_c =
                            (self.state.register_a >> self.state.combo(operand));
                    }
                }
            }
            if self.state.output == self.program.instructions {
                return i;
            }
            i += 1;
        }
    }
    fn get_next_output(&mut self) -> u8 {
        while let Some((opcode, operand)) = self.program.read_instruction() {
            match opcode {
                Operation::Adv => {
                    self.state.register_a >>= self.state.combo(operand);
                }
                Operation::Bxl => {
                    self.state.register_b ^= operand as Register;
                }
                Operation::Bst => {
                    self.state.register_b = self.state.combo(operand) % 8;
                }
                Operation::Jnz => {
                    if self.state.register_a != 0 {
                        self.program.instruction_pointer = operand as usize;
                    }
                }
                Operation::Bxc => {
                    self.state.register_b ^= self.state.register_c;
                }
                Operation::Out => {
                    return self.state.combo(operand) as u8 % 8;
                }
                Operation::Bdv => {
                    self.state.register_b = (self.state.register_a >> self.state.combo(operand));
                }
                Operation::Cdv => {
                    self.state.register_c = (self.state.register_a >> self.state.combo(operand));
                }
            }
        }
        unreachable!()
    }

    pub fn find_initial_a2(&mut self) -> Register {
        fn get_next_digit(
            computer: &mut Computer,
            current_a: Register,
            target: &[u8],
        ) -> Option<Register> {
            println!(
                "get_next_digit: current_a {:?}, target {:?}",
                current_a, target
            );
            if target.is_empty() {
                return Some(current_a);
            }
            for i in 0..=7 {
                computer.reset(current_a * 8 + i);
                if computer.get_next_output() == target[0] {
                    if let Some(new_a) = get_next_digit(computer, current_a * 8 + i, &target[1..]) {
                        return Some(new_a);
                    }
                }
            }
            None
        }
        get_next_digit(self, 0, &self.program.instructions.clone()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_dissasembly() {
        let mut computer = Computer::new(crate::TESTINPUT);
        println!("{}", computer.program.dissasemble());
        let mut computer = Computer::new(crate::TESTINPUT);
        let output = computer.execute_program();
        println!("{}", output);
    }
    #[test]
    fn test_dissasembly2() {
        let mut computer = Computer::new(crate::TESTINPUT2);
        println!("{}", computer.program.dissasemble());
        let mut computer = Computer::new(crate::TESTINPUT2);
        let output = computer.execute_program();
        println!("{}", output);
    }

    #[test]
    fn test_dissasembly3() {
        let mut computer = Computer::new(crate::INPUT);
        println!("{}", computer.program.dissasemble());
        let mut computer = Computer::new(crate::INPUT);
        let output = computer.execute_program();
        println!("{}", output);
    }

    #[test]
    fn test_find_a() {
        let mut computer = Computer::new(TESTINPUT2);
        println!("{}", computer.find_initial_a2());
    }
}
