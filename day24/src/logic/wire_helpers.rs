#[allow(dead_code)]
use super::Operation;

use std::fmt::Debug;

use std::ops::Deref;

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
    Value([bool; NO_CASES]),
    Connection {
        input1: T,
        input2: T,
        operation: Operation,
    },
}

impl<T, const NO_CASES: usize> Default for WireValue<T, NO_CASES> {
    fn default() -> Self {
        Self::Value([false; NO_CASES])
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Copy, PartialOrd, Ord)]
pub(crate) struct WireAnalytics {
    pub(crate) gates: GateFlags,
}

impl WireAnalytics {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            gates: self.gates.merge(&other.gates),
        }
    }
}

// a set of bits indicating whether gate n is included in a set
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GateFlags(pub [u128; 2]);

impl GateFlags {
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
    pub fn merge(&self, other: &Self) -> GateFlags {
        GateFlags([self.0[0] | other.0[0], self.0[1] | other.0[1]])
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
    use super::{GateFlags, WireName};

    fn flags_from_ints(a: u128, b: u128) -> GateFlags {
        GateFlags([a, b])
    }
    fn ints_from_flags(flags: GateFlags) -> (u128, u128) {
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

        let mut flags = GateFlags::default();
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
