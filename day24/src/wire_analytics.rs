use crate::{bit_array::LargeBitArray, errors::MachineError};

use super::wire::WireName;

use crate::bit_array::BitArray;

#[derive(Clone, PartialEq, Eq, Debug, Default, Copy, PartialOrd, Ord)]
pub(crate) struct WireAnalytics {
    pub(crate) wire_type: WireType,
    pub(crate) gate_array: LargeBitArray,
    pub(crate) x_bits_used: BitArray<u64>,
    pub(crate) y_bits_used: BitArray<u64>,
    pub(crate) generation: u8,
    pub(crate) highest_input_bit: u8,
    pub(crate) lowest_output_bit: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputWireType {
    X,
    Y,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, PartialOrd, Ord, Default)]
pub enum WireType {
    Input(InputWireType),
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
            highest_input_bit: self.highest_input_bit.max(other.highest_input_bit),
            lowest_output_bit: self.lowest_output_bit.max(other.lowest_output_bit),
        }
    }
    pub fn new_input_wire(wire_type: WireType, bit: usize) -> Self {
        let mut x_bits_used = BitArray::new();
        let mut y_bits_used = BitArray::new();
        match wire_type {
            WireType::Input(InputWireType::X) => x_bits_used.set(bit),
            WireType::Input(InputWireType::Y) => y_bits_used.set(bit),
            _ => {}
        }
        Self {
            wire_type,
            gate_array: LargeBitArray::default(),
            x_bits_used,
            y_bits_used,
            generation: 0,
            highest_input_bit: bit as u8,
            lowest_output_bit: u8::MAX,
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
            gate_array: LargeBitArray::default(),
            x_bits_used: BitArray::new(),
            y_bits_used: BitArray::new(),
            generation: 0,
            highest_input_bit: 0,
            lowest_output_bit: u8::MAX,
        }
    }
    pub fn validate(&self) -> Result<(), MachineError> {
        match self.wire_type {
            WireType::Input(InputWireType::X) => {
                if (1 << self.highest_input_bit) & self.x_bits_used.0 == 0 {
                    return Err(MachineError::LogicError(format!(
                        "highest_bit is not set in x_bits_used: highest bit {:?} != x_bits_used {:?}",
                        self.highest_input_bit, self.x_bits_used
                    )));
                }
            }
            WireType::Input(InputWireType::Y) => {
                if (1 << self.highest_input_bit) & self.y_bits_used.0 == 0 {
                    return Err(MachineError::LogicError(format!(
                        "highest_bit is not set in y_bits_used: highest bit {:?} != y_bits_used {:?}",
                        self.highest_input_bit, self.y_bits_used
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
