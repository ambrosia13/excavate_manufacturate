

pub mod excavatemanufacturate_blocks;
pub mod registry;
pub mod static_block_data;

pub type BlockData = Option<BlockType>;

// TODO: u8 id is efficient now, but may be problematic later
pub type BlockId = u8;

#[derive(Debug, Clone)]
pub struct BlockType {
    pub id: BlockId,
    // An entity ID, serves as a pointer to dynamic data unique to this block.
    //pub dynamic_data: Option<Entity>,
}

impl BlockType {
    pub const fn new_static(id: BlockId) -> Self {
        Self {
            id,
            //dynamic_data: None,
        }
    }
}
