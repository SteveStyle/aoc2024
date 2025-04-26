use crate::errors::MachineError;

use crate::errors::Result;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Wire<const N: usize> {
    pub wire_name: WireName,
    pub wire_index: usize,
    pub(crate) value_start: WireValue<usize, N>,
    pub(crate) value_calc: WireValue<usize, N>,
    pub wire_analytics: WireAnalytics,
}

impl<const N: usize> PartialOrd for Wire<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        debug_assert!(!(self.depends_on(other) && other.depends_on(self)));
        if self.depends_on(other) {
            Some(std::cmp::Ordering::Greater)
        } else if other.depends_on(self) {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl<const N: usize> Wire<N> {
    pub fn depends_on(&self, other: &Self) -> bool {
        self.wire_analytics.gate_array.get(other.wire_index)
    }

    pub fn validate(&self) -> Result<()> {
        self.wire_analytics.validate()?;
        if self.wire_analytics.gate_array.get(self.wire_index) {
            return Err(MachineError::LogicError(format!(
                "Wire {} depends on itself",
                self.wire_name.as_string()
            )));
        }

        Ok(())
    }
}

use crate::bit_array::BitArray;
use crate::machine::Operation;
use crate::wire_analytics::WireAnalytics;

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
    pub fn from_char_bit(start_char: u8, bit: u8) -> Self {
        let mut name = [0; 3];
        name[0] = start_char;
        name[1] = bit / 10 + b'0';
        name[2] = (bit % 10) + b'0';
        Self(name)
    }
    pub fn as_string(&self) -> String {
        self.0.iter().map(|b| *b as char).collect()
    }
    pub fn bit_index(&self) -> Option<u8> {
        match self[0] {
            b'x' | b'y' | b'z' => Some((self.0[1] - b'0') * 10 + (self.0[2] - b'0')),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord)]
pub enum WireValue<T, const NO_CASES: usize> {
    Value(BitArray<u128>),
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
