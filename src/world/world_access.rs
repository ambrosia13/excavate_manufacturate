use bevy::{prelude::*, utils::HashMap};

use crate::util::{block_pos::BlockPos, chunk_pos::ChunkPos};

use super::{block::BlockData, chunk::ChunkData};

#[derive(Resource)]
pub struct ExcavateManufacturateWorld {
    chunks: HashMap<ChunkPos, ChunkData>,
}

impl ExcavateManufacturateWorld {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&self, chunk_pos: ChunkPos) -> Option<&ChunkData> {
        self.chunks.get(&chunk_pos)
    }

    pub fn get_chunk_mut(&mut self, chunk_pos: ChunkPos) -> Option<&mut ChunkData> {
        self.chunks.get_mut(&chunk_pos)
    }

    pub fn insert_chunk(
        &mut self,
        chunk_pos: ChunkPos,
        chunk_data: ChunkData,
    ) -> Option<ChunkData> {
        self.chunks.insert(chunk_pos, chunk_data)
    }

    pub fn chunk_exists(&self, chunk_pos: ChunkPos) -> bool {
        self.chunks.contains_key(&chunk_pos)
    }

    pub fn get_block(&self, block_pos: BlockPos) -> Option<&BlockData> {
        if let Some(chunk_data) = self.get_chunk(ChunkPos::from(block_pos)) {
            Some(chunk_data.get(block_pos))
        } else {
            None
        }
    }

    pub fn get_block_mut(&mut self, block_pos: BlockPos) -> Option<&mut BlockData> {
        if let Some(chunk_data) = self.get_chunk_mut(ChunkPos::from(block_pos)) {
            Some(chunk_data.get_mut(block_pos))
        } else {
            None
        }
    }

    pub fn take_block(&mut self, block_pos: BlockPos) -> Option<BlockData> {
        self.get_chunk_mut(ChunkPos::from(block_pos))
            .map(|chunk_data| chunk_data.get_mut(block_pos).take())
    }

    /// Attempts to set the block at the position. If the block's chunk does not exist, nothing happens and the function returns false.
    pub fn set_block(&mut self, block_pos: BlockPos, block_data: BlockData) -> bool {
        if let Some(block) = self.get_block_mut(block_pos) {
            *block = block_data;
            true
        } else {
            false
        }
    }
}

pub fn setup_world_access(mut commands: Commands) {
    commands.insert_resource(ExcavateManufacturateWorld::new());
    info!("Initialized world");
}

pub fn remove_world_access(mut commands: Commands) {
    commands.remove_resource::<ExcavateManufacturateWorld>();
    info!("Removed world");
}
