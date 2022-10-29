use repr::Instruction;

pub trait Backend {
    fn lower(source: Vec<Instruction>) -> Self;
}
