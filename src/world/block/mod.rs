use bevy::prelude::*;

pub mod excavatemanufacturate_blocks;
pub mod registry;
pub mod static_block_data;

pub type BlockData = Option<BlockType>;

#[derive(Debug, Clone)]
pub struct BlockType {
    pub id: u8, // TODO: small id is efficient now, but may be problematic later
    pub dynamic_data: Option<Entity>,
}

impl BlockType {
    pub const fn new_static(id: u8) -> Self {
        Self {
            id,
            dynamic_data: None,
        }
    }
}
