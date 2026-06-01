use std::fmt;

pub type BlockId = u32;

#[derive(Debug)]
pub struct BasicBlock<I, T> {
    pub id: BlockId,
    pub instrs: Vec<I>,
    pub term: T,
}

impl<I: fmt::Display, T: fmt::Display> fmt::Display for BasicBlock<I, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  block_{}:\n", self.id)?;
        for instr in &self.instrs {
            write!(f, "    {}\n", instr)?;
        }
        write!(f, "    {}", self.term)
    }
}
