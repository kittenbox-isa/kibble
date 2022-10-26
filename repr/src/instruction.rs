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
