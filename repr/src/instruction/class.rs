use crate::Immediate;
use crate::Register;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Load {
    dest: Register,
    addr: Register,
    offset: Immediate<15>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lda {
    dest: Register,
    addr: Immediate<20>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ldi {
    dest: Register,
    mcd: Immediate<4>,
    imm: Immediate<16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reg {
    dest: Register,
    mcd: Immediate<4>,
    rs: Register,
    rt: Register,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Jal {
    link: Register,
    addr: Register,
    rs: Register,
    rt: Register,
    cc: Immediate<5>,
}
