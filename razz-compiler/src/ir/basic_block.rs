pub type BlockId = u32;

#[derive(Debug)]
pub struct BasicBlock<I, T> {
    pub id: BlockId,
    pub instrs: Vec<I>,
    pub term: T,
}
