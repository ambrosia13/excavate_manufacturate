use bevy::prelude::*;

pub mod excavatemanufacturate_blocks;
pub mod registry;
pub mod static_block_data;

pub type BlockData = Option<BlockType>;

#[derive(Debug)]
pub struct BlockType {
    pub id: &'static str,
    pub dynamic_data: Option<Entity>,
}

impl BlockType {
    pub const fn new_static(id: &'static str) -> Self {
        Self {
            id,
            dynamic_data: None,
        }
    }
}
