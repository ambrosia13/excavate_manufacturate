use std::hash::Hash;

use bevy::prelude::*;

use self::registry::BlockRegistry;

pub mod excavatemanufacturate_blocks;
pub mod registry;
pub mod static_block_data;

/// The data contained in the world. A newtype of `Option<BlockData>`, contains either a block or no block.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct BlockData(Option<Block>);

impl BlockData {
    /// Block data exists.
    pub fn some(block_type: Block) -> Self {
        Self(Some(block_type))
    }

    /// No block data.
    pub fn none() -> Self {
        Self(None)
    }
}

#[derive(Debug, Clone, Copy, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockName(pub &'static str);

/// Represents an identifier of static block data. To represent static data, individual blocks need to store this ID instead of the data itself.
#[derive(Debug, Clone, Copy, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub u16);

/// The data representation of a single block.
#[derive(Debug, Clone)]
pub struct Block {
    /// A static ID that serves as a pointer to static block data applicable to all blocks of this type.
    pub id: BlockId,

    /// An entity ID that serves as a pointer to dynamic data unique to this block.
    pub dynamic_data: Option<Entity>,
}

impl Block {
    pub fn from_name(name: &BlockName, registry: &BlockRegistry) -> Option<Self> {
        registry.get_block_id(name).map(|id| Self {
            id,
            dynamic_data: None,
        })
    }
}
