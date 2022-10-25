use kibble_intern::IString;

use crate::Immediate;
use crate::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    LDLI {
        dest: Register,
        imm: Immediate<20>,
    },
    LOAD {
        dest: Register,
        addr: Register,
        offs: Immediate<15>,
    },
}

impl Instruction {
    pub fn encoding(self) -> u32 {
        use Instruction::*;

        let mut bits = 0;

        match self {
            LDLI { dest, imm } => {
                bits |= *self.opcode();
            }
            LOAD { dest, addr, offs } => {
                bits |= *self.opcode();
                bits |= *dest.encoding() << 20;
                bits |= *addr.encoding() << 15;
                bits |= *offs;
            }
        }

        bits
    }

    fn opcode(self) -> Immediate<7> {
        use Instruction::*;

        let bits = match self {
            LDLI { .. } => 0b0000000,
            LOAD { .. } => 0b0000010,
        };

        Immediate::new(bits)
    }
}
