use std::{hash::Hash, num::NonZeroU16};

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::util::block_pos::BlockPos;

use self::registry::BlockRegistry;

pub mod dynamic_block_data;
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
///
/// Should never be constructed manually; this value is managed by the block registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub(in crate::world::block) NonZeroU16);

/// A lightweight data representation of a single block. Construction of this type is usually handled by the [`BlockRegistry`].
#[derive(Debug, Clone)]
pub struct Block {
    /// A static ID that serves as a pointer to static block data applicable to all blocks of this type.
    pub id: BlockId,

    /// An entity ID that serves as a pointer to dynamic data unique to this block.
    pub dynamic_data: Option<Entity>,
}

impl Block {
    /// Attaches dynamic data to this block using the entity ID of the dynamic data.
    pub fn with_dynamic_data(mut self, entity: Entity) -> Self {
        self.dynamic_data = Some(entity);
        self
    }
}
