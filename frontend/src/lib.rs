use repr::Instruction;

pub trait Frontend {
    fn lower(self) -> Vec<Instruction>;
}
