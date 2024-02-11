use bevy::{prelude::*, utils::HashMap};

use crate::util::{block_pos::BlockPos, chunk_pos::ChunkPos};

use super::{block::BlockData, chunk::ChunkData, CHUNK_SIZE_PADDED};

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
            // Update the chunk the block's in
            *block = block_data.clone();

            // // Also update neighboring chunk info, since neighboring chunks contain extra padding
            // let chunk_pos = ChunkPos::from(block_pos);
            // let offset_pos = ChunkData::offset_from_block_pos(block_pos);

            // let chunk_size_padded = CHUNK_SIZE_PADDED as i32;

            // // (chunk offset, vector index, offset value, neighbor offset value)
            // let neighbor_chunk_info = [
            //     (ChunkPos::new(0, 1, 0), 1, 0, chunk_size_padded),
            //     (ChunkPos::new(0, -1, 0), 1, chunk_size_padded, 0),
            //     (ChunkPos::new(1, 0, 0), 0, 0, chunk_size_padded),
            //     (ChunkPos::new(-1, 0, 0), 0, chunk_size_padded, 0),
            //     (ChunkPos::new(0, 0, 1), 2, 0, chunk_size_padded),
            //     (ChunkPos::new(0, 0, -1), 2, chunk_size_padded, 0),
            // ];

            // for (chunk_offset, vector_index, offset_value, neighbor_offset_value) in
            //     neighbor_chunk_info
            // {
            //     let neighbor_chunk_pos = chunk_pos + chunk_offset;
            //     if neighbor_chunk_pos[vector_index] == offset_value {
            //         if let Some(chunk) = self.get_chunk_mut(neighbor_chunk_pos) {
            //             let mut offset_pos = offset_pos;
            //             offset_pos[vector_index] = neighbor_offset_value;

            //             chunk.set_from_raw_offset(offset_pos, block_data.clone());
            //         }
            //     }
            // }

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
