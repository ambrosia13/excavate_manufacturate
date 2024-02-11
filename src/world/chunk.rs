use crate::util::{self, block_pos::BlockPos, chunk_pos::ChunkPos, mesh::ChunkMeshBuilder};

use super::{
    block::{registry::BlockRegistry, BlockData},
    world_access::ExcavateManufacturateWorld,
    CHUNK_SIZE,
};
use bevy::prelude::*;

pub struct ChunkData {
    blocks: Vec<BlockData>,
    scale: i32,
}

impl ChunkData {
    pub fn empty(scale: i32) -> Self {
        let chunk_size = Self::get_scaled_chunk_size(scale);

        Self {
            blocks: vec![BlockData::None; chunk_size * chunk_size * chunk_size],
            scale,
        }
    }

    pub fn with_data<F: FnMut(BlockPos) -> BlockData>(scale: i32, mut supplier: F) -> Self {
        let chunk_size = Self::get_scaled_chunk_size(scale);

        Self {
            blocks: (0..(chunk_size * chunk_size * chunk_size))
                .map(|i| supplier(BlockPos::from(Self::deindexify(i, scale) * scale)))
                .collect(),
            scale,
        }
    }

    pub fn get_scaled_chunk_size(scale: i32) -> usize {
        CHUNK_SIZE / scale as usize
    }

    pub fn for_each_mut<F: FnMut(BlockPos, &mut BlockData)>(&mut self, mut f: F) {
        self.blocks
            .iter_mut()
            .enumerate()
            .for_each(|(index, block_data)| {
                let offset = Self::deindexify(index, self.scale);
                let block_pos = BlockPos::from(offset * self.scale);

                f(block_pos, block_data);
            });
    }

    pub fn get_raw_array(&self) -> &[BlockData] {
        &self.blocks
    }

    pub fn indexify(offset: IVec3, scale: i32) -> usize {
        let scaled_chunk_size = Self::get_scaled_chunk_size(scale);

        (offset.z as usize * scaled_chunk_size * scaled_chunk_size)
            + (offset.y as usize * scaled_chunk_size)
            + offset.x as usize
    }

    pub fn deindexify(index: usize, scale: i32) -> IVec3 {
        let scaled_chunk_size = Self::get_scaled_chunk_size(scale);

        let z = index / (scaled_chunk_size * scaled_chunk_size);
        let y = (index % (scaled_chunk_size * scaled_chunk_size)) / scaled_chunk_size;
        let x = index % scaled_chunk_size;

        IVec3::new(x as i32, y as i32, z as i32)
    }

    pub fn get(&self, block_pos: BlockPos) -> &BlockData {
        let offset = block_pos.as_chunk_offset().inner() / self.scale;
        &self.blocks[Self::indexify(offset, self.scale)]
    }

    pub fn get_mut(&mut self, block_pos: BlockPos) -> &mut BlockData {
        let offset = block_pos.as_chunk_offset().inner() / self.scale;
        &mut self.blocks[Self::indexify(offset, self.scale)]
    }

    pub fn try_get(&self, block_pos: BlockPos) -> Option<&BlockData> {
        let offset = block_pos.as_chunk_offset().inner() / self.scale;
        self.blocks.get(Self::indexify(offset, self.scale))
    }

    pub fn set(&mut self, block_pos: BlockPos, block: BlockData) {
        let offset = block_pos.as_chunk_offset().inner() / self.scale;
        self.blocks[Self::indexify(offset, self.scale)] = block;
    }

    pub fn get_from_raw_offset(&self, offset: IVec3) -> &BlockData {
        &self.blocks[Self::indexify(offset, self.scale)]
    }

    pub fn try_get_from_raw_offset(&self, offset: IVec3) -> Option<&BlockData> {
        self.blocks.get(Self::indexify(offset, self.scale))
    }

    pub fn set_from_raw_offset(&mut self, offset: IVec3, block: BlockData) {
        self.blocks[Self::indexify(offset, self.scale)] = block;
    }

    pub fn get_mesh(
        &self,
        chunk_pos: ChunkPos,
        block_registry: &BlockRegistry,
        world: &ExcavateManufacturateWorld,
    ) -> Mesh {
        let mut mesh_builder = ChunkMeshBuilder::new();

        let scale = self.scale;
        let scaled_chunk_size = Self::get_scaled_chunk_size(scale) as i32;

        for x in 0..scaled_chunk_size {
            for y in 0..scaled_chunk_size {
                for z in 0..scaled_chunk_size {
                    let offset = IVec3::new(x, y, z);
                    let index = Self::indexify(offset, scale);

                    let block = &self.blocks[index];

                    let BlockData::Some(block_type) = block else {
                        continue;
                    };

                    for ((dx, dy, dz), face, normals, uvs) in util::mesh::NEIGHBOR_DATA {
                        let neighbor_pos = offset + IVec3::new(dx, dy, dz);

                        let neighbor_exists_in_chunk = neighbor_pos.cmpge(IVec3::splat(0)).all()
                            && neighbor_pos
                                .cmple(IVec3::splat(scaled_chunk_size - 1))
                                .all();

                        let add_face = if neighbor_exists_in_chunk {
                            // We can get occlusion info from the chunk data itself
                            self.try_get_from_raw_offset(neighbor_pos)
                                .is_some_and(|block_data| block_data.is_none())
                        } else {
                            let neighbor_pos = (offset + IVec3::new(dx, dy, dz)) * scale;
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
                                face,
                                normals,
                                uvs,
                                offset.as_vec3() * scale as f32,
                                scale as f32,
                                static_block_data.atlas_coordinates,
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
