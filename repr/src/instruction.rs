use class::*;

pub mod class;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    LDLI(Ldi),
    LDUI(Ldi),
    LDLZ(Ldi),
    LDUZ(Ldi),
    LDLS(Ldi),
    LDUS(Ldi),
    LDA(Lda),
    LOAD(Load),
    STA(Ldi),
    STOR(Load),
    ADD(Reg),
    SUB(Reg),
    OR(Reg),
    AND(Reg),
    XOR(Reg),
    NOR(Reg),
    JAL(Jal),
}
