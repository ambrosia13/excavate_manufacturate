use crate::{
    util::{self, block_pos::BlockPos, mesh::ChunkMeshBuilder},
    world::CHUNK_SIZE_INT,
};

use super::{block::BlockData, CHUNK_SIZE_PADDED};
use bevy::prelude::*;

pub struct ChunkData {
    blocks: [BlockData; CHUNK_SIZE_PADDED * CHUNK_SIZE_PADDED * CHUNK_SIZE_PADDED],
}

impl ChunkData {
    pub fn empty() -> Self {
        Self {
            blocks: std::array::from_fn(|_| BlockData::Empty),
        }
    }

    pub fn with_data<F: FnMut(BlockPos) -> BlockData>(mut supplier: F) -> Self {
        Self {
            blocks: std::array::from_fn(|i| {
                let offset = Self::deindexify(i);
                supplier(Self::block_pos_from_offset(offset))
            }),
        }
    }

    pub fn for_each_mut<F: FnMut(BlockPos, &mut BlockData)>(&mut self, mut f: F) {
        self.blocks
            .iter_mut()
            .enumerate()
            .for_each(|(index, block_data)| {
                let offset = Self::deindexify(index);
                let block_pos = Self::block_pos_from_offset(offset);

                f(block_pos, block_data);
            });
    }

    pub fn get_raw_array(&self) -> &[BlockData] {
        &self.blocks
    }

    pub fn block_pos_from_offset(offset: IVec3) -> BlockPos {
        // (1, 1, 1) -> (0, 0, 0)
        BlockPos::from(offset - 1)
    }

    pub fn offset_from_block_pos(block_pos: BlockPos) -> IVec3 {
        // (0, 0, 0) -> (1, 1, 1)
        block_pos.as_chunk_offset().inner() + 1
    }

    pub fn indexify(offset: IVec3) -> usize {
        (offset.z as usize * CHUNK_SIZE_PADDED * CHUNK_SIZE_PADDED)
            + (offset.y as usize * CHUNK_SIZE_PADDED)
            + offset.x as usize
    }

    pub fn deindexify(index: usize) -> IVec3 {
        let z = index / (CHUNK_SIZE_PADDED * CHUNK_SIZE_PADDED);
        let y = (index % (CHUNK_SIZE_PADDED * CHUNK_SIZE_PADDED)) / CHUNK_SIZE_PADDED;
        let x = index % CHUNK_SIZE_PADDED;

        IVec3::new(x as i32, y as i32, z as i32)
    }

    pub fn get(&self, block_pos: BlockPos) -> &BlockData {
        let offset = Self::offset_from_block_pos(block_pos);
        &self.blocks[Self::indexify(offset)]
    }

    pub fn get_mut(&mut self, block_pos: BlockPos) -> &mut BlockData {
        let offset = Self::offset_from_block_pos(block_pos);
        &mut self.blocks[Self::indexify(offset)]
    }

    pub fn try_get(&self, block_pos: BlockPos) -> Option<&BlockData> {
        let offset = Self::offset_from_block_pos(block_pos);
        self.blocks.get(Self::indexify(offset))
    }

    pub fn set(&mut self, block_pos: BlockPos, block: BlockData) {
        let offset = Self::offset_from_block_pos(block_pos);
        self.blocks[Self::indexify(offset)] = block;
    }

    pub fn get_from_raw_offset(&self, offset: IVec3) -> &BlockData {
        &self.blocks[Self::indexify(offset)]
    }

    pub fn try_get_from_raw_offset(&self, offset: IVec3) -> Option<&BlockData> {
        self.blocks.get(Self::indexify(offset))
    }

    pub fn set_from_raw_offset(&mut self, offset: IVec3, block: BlockData) {
        self.blocks[Self::indexify(offset)] = block;
    }

    pub fn get_mesh(&self) -> Mesh {
        let mut mesh_builder = ChunkMeshBuilder::new();

        for x in 1..(CHUNK_SIZE_INT + 1) {
            for y in 1..(CHUNK_SIZE_INT + 1) {
                for z in 1..(CHUNK_SIZE_INT + 1) {
                    let offset = IVec3::new(x, y, z);
                    let index = Self::indexify(offset);

                    let offset_without_padding = Self::block_pos_from_offset(offset).inner();

                    let block = &self.blocks[index];

                    // if block has no geometry, don't add faces for it
                    if !block.has_geometry() {
                        continue;
                    }

                    for ((dx, dy, dz), face, normals, uvs) in util::mesh::NEIGHBOR_DATA {
                        let neighbor_pos = offset + IVec3::new(dx, dy, dz);

                        if let Some(neighbor) = self.try_get_from_raw_offset(neighbor_pos) {
                            if !neighbor.is_opaque() {
                                mesh_builder.add_face(
                                    face,
                                    normals,
                                    uvs,
                                    offset_without_padding.as_vec3(),
                                );
                            }
                        }
                    }
                }
            }
        }

        mesh_builder.as_mesh()
    }
}

pub struct ChunkOcclusionData {
    blocks: [bool; CHUNK_SIZE_PADDED * CHUNK_SIZE_PADDED * CHUNK_SIZE_PADDED],
}

impl ChunkOcclusionData {
    pub fn from_chunk(chunk_data: &ChunkData) -> Self {
        let chunk_array = chunk_data.get_raw_array();

        Self {
            blocks: std::array::from_fn(|i| chunk_array[i].is_opaque()),
        }
    }

    pub fn try_get_from_raw_offset(&self, offset: IVec3) -> Option<bool> {
        self.blocks.get(ChunkData::indexify(offset)).copied()
    }

    pub fn get_mesh(&self) -> Mesh {
        let mut mesh_builder = ChunkMeshBuilder::new();

        for x in 1..(CHUNK_SIZE_INT + 1) {
            for y in 1..(CHUNK_SIZE_INT + 1) {
                for z in 1..(CHUNK_SIZE_INT + 1) {
                    let offset = IVec3::new(x, y, z);
                    let index = ChunkData::indexify(offset);

                    let offset_without_padding = ChunkData::block_pos_from_offset(offset).inner();

                    let block = self.blocks[index];

                    // if block has no geometry, don't add faces for it
                    if !block {
                        continue;
                    }

                    for ((dx, dy, dz), face, normals, uvs) in util::mesh::NEIGHBOR_DATA {
                        let neighbor_pos = offset + IVec3::new(dx, dy, dz);

                        if let Some(neighbor) = self.try_get_from_raw_offset(neighbor_pos) {
                            if !neighbor {
                                mesh_builder.add_face(
                                    face,
                                    normals,
                                    uvs,
                                    offset_without_padding.as_vec3(),
                                );
                            }
                        }
                    }
                }
            }
        }

        mesh_builder.as_mesh()
    }
}
