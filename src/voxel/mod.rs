use bevy::prelude::*;

pub mod chunk;
pub mod gen;
pub mod voxels;
pub mod world;

pub const CHUNK_SIZE: usize = 16;
pub const PADDED_CHUNK_SIZE: usize = CHUNK_SIZE + 2;

pub type WorldPos = IVec3;
pub type ChunkPos = IVec3;

pub fn world_pos_to_chunk_pos(world_pos: WorldPos) -> ChunkPos {
    world_pos / CHUNK_SIZE as i32
}

pub fn chunk_pos_to_world_pos(chunk_pos: ChunkPos) -> WorldPos {
    chunk_pos * CHUNK_SIZE as i32
}

#[derive(PartialEq, Eq)]
pub enum VoxelShape {
    Empty,
    Default,
    Custom(),
}

pub trait Voxel: Sync + Send {
    fn is_opaque(&self) -> bool;
    fn get_voxel_shape(&self) -> VoxelShape;
    fn has_no_geometry(&self) -> bool {
        self.get_voxel_shape() == VoxelShape::Empty
    }

    fn on_destroy(&mut self) {}
    fn tick(&mut self) {}
}
