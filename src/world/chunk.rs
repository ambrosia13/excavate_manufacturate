use crate::util::{self, block_pos::BlockPos, chunk_pos::ChunkPos, mesh::ChunkMeshBuilder};

use super::{
    block::{registry::BlockRegistry, BlockData},
    world_access::ExcavateManufacturateWorld,
    CHUNK_SIZE,
};
use bevy::prelude::*;

pub struct ChunkData {
    blocks: Vec<BlockData>,
    num_blocks: u32,
}

impl ChunkData {
    pub fn empty() -> Self {
        Self {
            blocks: vec![BlockData::None; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            num_blocks: 0,
        }
    }

    pub fn with_data<F: FnMut(BlockPos) -> BlockData>(mut supplier: F) -> Self {
        let mut num_blocks = 0;

        let blocks = (0..(CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE))
            .map(|i| {
                let data = supplier(BlockPos::from(Self::deindexify(i)));
                num_blocks += data.is_some() as u32; // If the block isn't empty, increment num_blocks

                data
            })
            .collect();

        Self { blocks, num_blocks }
    }

    pub fn is_empty(&self) -> bool {
        self.num_blocks == 0
    }

    pub fn get_raw_array(&self) -> &[BlockData] {
        &self.blocks
    }

    pub fn indexify(offset: IVec3) -> usize {
        (offset.z as usize * CHUNK_SIZE * CHUNK_SIZE)
            + (offset.y as usize * CHUNK_SIZE)
            + offset.x as usize
    }

    pub fn deindexify(index: usize) -> IVec3 {
        let z = index / (CHUNK_SIZE * CHUNK_SIZE);
        let y = (index % (CHUNK_SIZE * CHUNK_SIZE)) / CHUNK_SIZE;
        let x = index % CHUNK_SIZE;

        IVec3::new(x as i32, y as i32, z as i32)
    }

    pub fn get(&self, block_pos: BlockPos) -> &BlockData {
        let offset = block_pos.as_chunk_offset().inner();
        &self.blocks[Self::indexify(offset)]
    }

    pub fn set(&mut self, block_pos: BlockPos, block: BlockData) {
        if self.get(block_pos).is_some() {
            // The existing block is being replaced by air.
            if block.is_none() {
                self.num_blocks -= 1;
            }
        } else if block.is_some() {
            // A block is being placed in an empty spot, increasing the total amount.
            self.num_blocks += 1;
        }

        let offset = block_pos.as_chunk_offset().inner();
        self.blocks[Self::indexify(offset)] = block;
    }

    pub fn get_from_raw_offset(&self, offset: IVec3) -> &BlockData {
        &self.blocks[Self::indexify(offset)]
    }

    pub fn try_get_from_raw_offset(&self, offset: IVec3) -> Option<&BlockData> {
        self.blocks.get(Self::indexify(offset))
    }

    pub fn get_mesh(
        &self,
        chunk_pos: ChunkPos,
        block_registry: &BlockRegistry,
        world: &ExcavateManufacturateWorld,
    ) -> Mesh {
        let mut mesh_builder = ChunkMeshBuilder::new();

        for x in 0..(CHUNK_SIZE as i32) {
            for y in 0..(CHUNK_SIZE as i32) {
                for z in 0..(CHUNK_SIZE as i32) {
                    let offset = IVec3::new(x, y, z);
                    let index = Self::indexify(offset);

                    let block = &self.blocks[index];

                    let BlockData::Some(block_type) = block else {
                        continue;
                    };

                    for ((dx, dy, dz), geometry, normals, uvs, face) in util::mesh::NEIGHBOR_DATA {
                        let neighbor_pos = offset + IVec3::new(dx, dy, dz);

                        let neighbor_exists_in_chunk = neighbor_pos.cmpge(IVec3::splat(0)).all()
                            && neighbor_pos
                                .cmple(IVec3::splat(CHUNK_SIZE as i32 - 1))
                                .all();

                        let add_face = if neighbor_exists_in_chunk {
                            // We can get occlusion info from the chunk data itself
                            self.try_get_from_raw_offset(neighbor_pos)
                                .is_some_and(|block_data| block_data.is_none())
                        } else {
                            let neighbor_pos = offset + IVec3::new(dx, dy, dz);
                            let world_neighbor_pos =
                                BlockPos::from(chunk_pos) + BlockPos::from(neighbor_pos);

                            // Access the world data structure for occlusion test
                            // Equivalent to, like, an Option::is_none_or() if it actually existed
                            !world
                                .get_block(world_neighbor_pos)
                                .is_some_and(|block_data| block_data.is_some())
                        };

                        if add_face {
                            let static_block_data = block_registry.get_block_data(block_type.id);

                            mesh_builder.add_face(
                                geometry,
                                normals,
                                uvs,
                                offset.as_vec3(),
                                1.0,
                                static_block_data.textures.get_coords(face),
                                block_registry.atlas_size,
                            );
                        }
                    }
                }
            }
        }

        mesh_builder.as_mesh()
    }
}
