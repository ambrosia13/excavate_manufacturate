use std::hash::Hash;

use bevy::prelude::*;

pub mod excavatemanufacturate_blocks;
pub mod registry;
pub mod static_block_data;

/// The data contained in the world. A newtype of `Option<BlockData>`, contains either a block or no block.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct BlockData(Option<BlockType>);

impl BlockData {
    /// Block data exists.
    pub fn some(block_type: BlockType) -> Self {
        Self(Some(block_type))
    }

    /// No block data.
    pub fn none() -> Self {
        Self(None)
    }

    /// Converts this [`BlockData`] into an `Option<BlockType>`.
    pub fn get(self) -> Option<BlockType> {
        self.0
    }
}

/// Represents an identifier of static block data. To represent static data, individual blocks need to store this ID instead of the data itself.
#[derive(Debug, Clone, Copy, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub &'static str);

/// The data representation of a single block.
#[derive(Debug, Clone)]
pub struct BlockType {
    /// A static ID that serves as a pointer to static block data applicable to all blocks of this type.
    pub id: BlockId,

    /// An entity ID that serves as a pointer to dynamic data unique to this block.
    pub dynamic_data: Option<Entity>,
}

impl BlockType {
    /// Create a block without any dynamic data. Should be used for block types that are identical across all instances.
    pub const fn new_static(id: BlockId) -> Self {
        Self {
            id,
            dynamic_data: None,
        }
    }
}
