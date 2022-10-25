use crate::Immediate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents a register on a KittenBox ISA CPU.
pub enum Register {
    /// The instruction pointer
    IP,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    R31,
}

impl Register {
    /// Retrieve the encoding of a register
    ///
    /// This method is used in the context of encoding instructions. It is
    /// guaranteed to return a value that is representable within 5 bits.
    pub fn encoding(self) -> Immediate<5> {
        use Register::*;

        let bits = match self {
            IP => 0b00000,
            R0 => 0b00000,
            R1 => 0b00001,
            R2 => 0b00010,
            R3 => 0b00011,
            R4 => 0b00100,
            R5 => 0b00101,
            R6 => 0b00110,
            R7 => 0b00111,
            R8 => 0b01000,
            R9 => 0b01001,
            R10 => 0b01010,
            R11 => 0b01011,
            R12 => 0b01100,
            R13 => 0b01101,
            R14 => 0b01110,
            R15 => 0b01111,
            R16 => 0b10000,
            R17 => 0b10001,
            R18 => 0b10010,
            R19 => 0b10011,
            R20 => 0b10100,
            R21 => 0b10101,
            R22 => 0b10110,
            R23 => 0b10111,
            R24 => 0b11000,
            R25 => 0b11001,
            R26 => 0b11010,
            R27 => 0b11011,
            R28 => 0b11100,
            R29 => 0b11101,
            R30 => 0b11110,
            R31 => 0b11111,
        };

        Immediate::new(bits)
    }
}
