pub type BlockId = u32;

pub struct BasicBlock<I, T> {
    pub id: BlockId,
    pub instrs: Vec<I>,
    pub term: T,
}
