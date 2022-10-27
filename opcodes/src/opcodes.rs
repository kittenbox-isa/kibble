#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Opcodes {
    pub ADD: i8,
    pub AND: i8,
    pub JAL: i8,
    pub LDA: i8,
    pub LDLI: i8,
    pub LDLS: i8,
    pub LDLZ: i8,
    pub LDUI: i8,
    pub LDUS: i8,
    pub LDUZ: i8,
    pub LOAD: i8,
    pub LSL: i8,
    pub LSR: i8,
    pub NOT: i8,
    pub OR: i8,
    pub STA: i8,
    pub STOR: i8,
    pub SUB: i8,
    pub XOR: i8,
}

pub const OPCODES: Opcodes = Opcodes {
    ADD: 4,
    AND: 7,
    JAL: 12,
    LDA: 0,
    LDLI: 0,
    LDLS: 0,
    LDLZ: 0,
    LDUI: 0,
    LDUS: 0,
    LDUZ: 0,
    LOAD: 1,
    LSL: 10,
    LSR: 11,
    NOT: 9,
    OR: 6,
    STA: 2,
    STOR: 3,
    SUB: 5,
    XOR: 8,
};
