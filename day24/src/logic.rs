use std::{collections::HashMap, fmt::Debug, ops::Deref};

#[derive(PartialEq, Copy, Clone, Hash, Eq, PartialOrd, Ord)]
pub struct WireName([u8; 3]);

impl Debug for WireName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.as_string()).finish()
    }
}

impl WireName {
    fn new(slice: &[u8]) -> Self {
        Self([slice[0], slice[1], slice[2]])
    }
    fn as_string(&self) -> String {
        self.0.iter().map(|b| *b as char).collect()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum WireValue {
    Value(bool),
    Connection {
        input1: WireName,
        input2: WireName,
        operation: Operation,
    },
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
pub struct Logic {
    wires: HashMap<WireName, WireValue>,
}

impl Logic {
    pub fn new(input: &str) -> Self {
        let mut split = input.split("\n\n");
        let input_values = split.next().unwrap();
        let input_connections = split.next().unwrap();

        let mut wires = HashMap::new();

        for line in input_values.lines() {
            let bytes = line.as_bytes();
            wires.insert(
                WireName::new(&bytes[0..3]),
                WireValue::Value(bytes[5] == b'1'),
            );
        }

        for line in input_connections.lines() {
            let mut bytes = line.as_bytes();
            let input1 = WireName::new(&bytes[0..3]);
            let operation = match &bytes[4..7] {
                b"AND" => Operation::AND,
                b"XOR" => Operation::XOR,
                b"OR " => Operation::OR,
                _ => unreachable!(),
            };

            if operation == Operation::OR {
                bytes = &bytes[7..];
            } else {
                bytes = &bytes[8..];
            }
            let input2 = WireName::new(&bytes[0..3]);
            let output = WireName::new(&bytes[7..]);
            wires.insert(
                output,
                WireValue::Connection {
                    input1,
                    input2,
                    operation,
                },
            );
        }

        Logic { wires }
    }

    fn eval(&mut self, wire_name: WireName) -> bool {
        let wire_value = self.wires.get(&wire_name).unwrap().to_owned();
        match wire_value {
            WireValue::Value(b) => b,
            WireValue::Connection {
                input1,
                input2,
                operation,
            } => {
                let input1 = self.eval(input1);
                let input2 = self.eval(input2);
                let value = match operation {
                    Operation::AND => input1 & input2,
                    Operation::OR => input1 | input2,
                    Operation::XOR => input1 ^ input2,
                };
                self.wires.insert(wire_name, WireValue::Value(value));
                value
            }
        }
    }

    fn analyse(&self) {
        let names: Vec<WireName> = self.wires.keys().copied().collect();
        let mut counts: HashMap<WireName, (usize, usize, usize)> = HashMap::new();
        for (&name, &value) in self.wires.iter() {
            match value {
                WireValue::Value(_) => {
                    let (initial, input, output) = counts.entry(name).or_insert((0, 0, 0));
                    *initial += 1;
                }
                WireValue::Connection {
                    input1,
                    input2,
                    operation,
                } => {
                    let (_, input, _) = counts.entry(input1).or_insert((0, 0, 0));
                    *input += 1;
                    let (_, input, _) = counts.entry(input2).or_insert((0, 0, 0));
                    *input += 1;
                    let (_, _, output) = counts.entry(name).or_insert((0, 0, 0));
                    *output += 1;
                }
            }
        }
        let mut v: Vec<(&WireName, &(usize, usize, usize))> = counts.iter().collect();
        v.sort();

        for (&name, (initial, input, output)) in v {
            println!("{name:?}  ({initial}, {input}, {output})");
        }
    }

    pub fn eval_all(&mut self) -> Ear {
        let mut ear = Ear::default();
        let mut zlist: Vec<WireName> = self.wires.keys().copied().collect();
        zlist.sort();
        zlist.reverse();
        for zname in zlist.into_iter().filter(|n| n.0[0] == b'z') {
            let value = self.eval(zname);
            ear.push(value, zname);
        }
        ear
    }
}

//Eval All Response type
#[derive(Debug, Default)]
pub struct Ear {
    value: usize,
    #[cfg(test)]
    values: Vec<(WireName, bool)>,
}

impl Deref for Ear {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Ear {
    fn push(&mut self, b: bool, name: WireName) {
        self.value <<= 1;
        self.value += b as usize;
        #[cfg(test)]
        self.values.push((name, b));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_new() {
        let logic = Logic::new(TESTINPUT);
        println!("{logic:#?}");
        logic.analyse();
    }
    #[test]
    fn test_eval_all() {
        let mut logic = Logic::new(TESTINPUT);
        logic.analyse();
        let ear = logic.eval_all();
        println!("{ear:?}");
        assert_eq!(*ear, 4);
    }

    #[test]
    fn test_eval_all2() {
        let mut logic = Logic::new(TESTINPUT2);
        logic.analyse();
        let ear = logic.eval_all();
        println!("{ear:?}");
        assert_eq!(*ear, 2024);
    }
    #[test]
    fn test_eval_all3() {
        let mut logic = Logic::new(INPUT);
        logic.analyse();
        let ear = logic.eval_all();
        println!("{ear:?}");
        // assert_eq!(*ear, 2024);
    }
}
