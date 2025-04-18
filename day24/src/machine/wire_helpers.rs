use crate::bit_array::BitArray;
use crate::errors::MachineError;

#[allow(dead_code)]
use super::Operation;

use std::fmt::Debug;

use std::ops::{BitAndAssign, BitOrAssign, Deref, Not};

#[derive(PartialEq, Copy, Clone, Hash, Eq, PartialOrd, Ord)]
pub struct WireName(pub [u8; 3]);

impl Default for WireName {
    fn default() -> Self {
        WireName([b'w', b'z' + 1, 0])
    }
}

impl Deref for WireName {
    type Target = [u8; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for WireName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.as_string()).finish()
    }
}

impl WireName {
    pub(crate) fn from_slice(slice: &[u8]) -> Self {
        Self([slice[0], slice[1], slice[2]])
    }
    pub fn from_char_bit(start_char: u8, bit: usize) -> Self {
        let bit = bit as u8;
        let mut name = [0; 3];
        name[0] = start_char;
        name[1] = bit / 10 + b'0';
        name[2] = (bit % 10) + b'0';
        Self(name)
    }
    pub fn as_string(&self) -> String {
        self.0.iter().map(|b| *b as char).collect()
    }
    pub fn bit_index(&self) -> Option<usize> {
        match self[0] {
            b'x' | b'y' | b'z' => {
                Some((self.0[1] - b'0') as usize * 10 + (self.0[2] - b'0') as usize)
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
pub enum WireValue<T, const NO_CASES: usize> {
    Value(BitArray),
    Gate {
        input1: T,
        input2: T,
        operation: Operation,
    },
}

impl<T, const NO_CASES: usize> Default for WireValue<T, NO_CASES> {
    fn default() -> Self {
        Self::Value(BitArray::new())
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Copy, PartialOrd, Ord)]
pub(crate) struct WireAnalytics {
    pub(crate) wire_type: WireType,
    pub(crate) gate_array: GateArray,
    pub(crate) x_bits_used: BitArray,
    pub(crate) y_bits_used: BitArray,
    pub(crate) generation: u8,
    pub(crate) highest_bit: u8,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord, Default)]
pub enum WireType {
    X,
    Y,
    #[default]
    GateOutput,
    Z,
}

impl WireAnalytics {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            wire_type: WireType::GateOutput,
            gate_array: self.gate_array.merge(&other.gate_array),
            x_bits_used: self.x_bits_used | other.x_bits_used,
            y_bits_used: self.y_bits_used | other.y_bits_used,
            generation: self.generation.max(other.generation) + 1,
            highest_bit: self.highest_bit.max(other.highest_bit),
        }
    }
    pub fn new_input_wire(wire_type: WireType, bit: usize) -> Self {
        let mut x_bits_used = BitArray::new();
        let mut y_bits_used = BitArray::new();
        match wire_type {
            WireType::X => x_bits_used.set(bit),
            WireType::Y => y_bits_used.set(bit),
            _ => {}
        }
        Self {
            wire_type,
            gate_array: GateArray::default(),
            x_bits_used,
            y_bits_used,
            generation: 0,
            highest_bit: bit as u8,
        }
    }
    pub fn new_gate_wire(wire_name: WireName) -> Self {
        let wire_type = match wire_name[0] {
            b'x' | b'y' => unreachable!(),
            b'z' => WireType::Z,
            _ => WireType::GateOutput,
        };

        Self {
            wire_type,
            gate_array: GateArray::default(),
            x_bits_used: BitArray::new(),
            y_bits_used: BitArray::new(),
            generation: 0,
            highest_bit: 0,
        }
    }
    pub fn validate(&self) -> Result<(), MachineError> {
        match self.wire_type {
            WireType::X => {
                if (1 << self.highest_bit) & self.x_bits_used.0 == 0 {
                    return Err(MachineError::LogicError(format!(
                        "highest_bit is not set in x_bits_used: highest bit {:?} != x_bits_used {:?}",
                        self.highest_bit, self.x_bits_used
                    )));
                }
            }
            WireType::Y => {
                if (1 << self.highest_bit) & self.y_bits_used.0 == 0 {
                    return Err(MachineError::LogicError(format!(
                        "highest_bit is not set in y_bits_used: highest bit {:?} != y_bits_used {:?}",
                        self.highest_bit, self.y_bits_used
                    )));
                }
            }
            _ => {
                if self.x_bits_used != self.y_bits_used {
                    return Err(MachineError::LogicError(format!(
                        "x_bits_used and y_bits_used are not equal: {:?} != {:?}",
                        self.x_bits_used, self.y_bits_used
                    )));
                }
            }
        }

        Ok(())
    }
}

// a set of bits indicating whether gate n is included in a set
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GateArray(pub [u128; 2]);

impl Not for GateArray {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self([!self.0[0], !self.0[1]])
    }
}

impl BitOrAssign for GateArray {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0[0] |= rhs.0[0];
        self.0[1] |= rhs.0[1];
    }
}

impl GateArray {
    pub fn set(&mut self, n: usize) {
        if n < 128 {
            self.0[1] |= 1 << n;
        } else if n < 256 {
            self.0[0] |= 1 << (n % 128);
        }
    }
    pub fn unset(&mut self, n: usize) {
        if n < 128 {
            self.0[1] &= !(1 << n);
        } else {
            self.0[0] &= !(1 << (n % 128));
        }
    }
    pub fn get(&self, n: usize) -> bool {
        if n < 128 {
            self.0[1] & (1 << n) != 0
        } else {
            self.0[0] & (1 << (n % 128)) != 0
        }
    }
    pub fn merge(&self, other: &Self) -> GateArray {
        GateArray([self.0[0] | other.0[0], self.0[1] | other.0[1]])
    }
    pub fn as_binary_string(&self) -> String {
        format!("{:0128b}{:0128b}", self.0[0], self.0[1])
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.0[0] == 0 && self.0[1] == 0
    }
}

#[cfg(test)]
mod tests {
    //test GateFlags
    use super::{GateArray, WireName};

    fn flags_from_ints(a: u128, b: u128) -> GateArray {
        GateArray([a, b])
    }
    fn ints_from_flags(flags: GateArray) -> (u128, u128) {
        (flags.0[0], flags.0[1])
    }

    #[test]
    fn test_gate_flags_merge() {
        let flags1 = flags_from_ints(0b101, 0b1000);
        let flags2 = flags_from_ints(0b1011, 0b10010);
        let flags3 = flags1.merge(&flags2);
        assert_eq!(ints_from_flags(flags3), (0b1111, 0b11010));
    }

    #[test]
    fn test_gate_flags() {
        let flags = flags_from_ints(123, 456);
        let (a, b) = ints_from_flags(flags);
        assert_eq!([a, b], [123, 456]);

        let mut flags = GateArray::default();
        assert_eq!(ints_from_flags(flags), (0, 0));
        flags.set(0);
        assert!(flags.get(0));
        assert_eq!(ints_from_flags(flags), (0, 1));
        flags.set(1);
        assert!(flags.get(0));
        assert!(flags.get(1));
        assert!(!flags.get(2));
        assert_eq!(ints_from_flags(flags), (0, 3));
        flags.set(127);
        assert!(flags.get(127));
        flags.unset(0);
        flags.unset(127);
        assert!(!flags.get(0));
        assert!(flags.get(1));
        assert!(!flags.get(2));
        assert_eq!(ints_from_flags(flags), (0, 2));
        flags.unset(1);
        assert!(!flags.get(0));
        assert!(!flags.get(1));
        assert!(!flags.get(2));
        assert_eq!(ints_from_flags(flags), (0, 0));
        let flags = flags_from_ints(1 << 127, 3 << 126);
        assert!(!flags.get(0));
        assert!(!flags.get(1));
        assert!(!flags.get(2));
        assert!(!flags.get(125));
        assert!(flags.get(126));
        assert!(flags.get(127));
        assert!(!flags.get(128));
        assert!(!flags.get(129));
        assert!(!flags.get(244));
        assert!(flags.get(255));
    }

    //test wirename
    #[test]
    fn test_level() {
        let wn = WireName::from_char_bit(b'x', 1);
        assert_eq!(wn.as_string(), "x01");
        assert_eq!(wn.bit_index(), Some(1));
    }
}
