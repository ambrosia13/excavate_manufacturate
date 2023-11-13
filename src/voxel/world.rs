use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use super::{chunk::VoxelChunk, gen::DefaultWorldGenerator, ChunkPos, Voxel, WorldPos};

#[derive(Resource)]
pub struct ViewDistance {
    pub distance: usize,
}

#[derive(Resource)]
pub struct VoxelWorld {
    chunks: HashMap<ChunkPos, VoxelChunk>,
    loaded_chunks: HashSet<ChunkPos>,
}

pub enum ChunkLoadingError {
    AlreadyCreated,
    AlreadyLoaded,
    NotCreatedYet,
}

impl VoxelWorld {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            loaded_chunks: HashSet::new(),
        }
    }

    /// Returns the chunk at the specified position, or None if it hasn't been created yet
    pub fn get_chunk(&mut self, pos: ChunkPos) -> Option<&mut VoxelChunk> {
        self.chunks.get_mut(&pos)
    }

    /// Generates the specified chunk, returning true if the chunk has already been generated
    pub fn create_chunk(&mut self, pos: ChunkPos) -> bool {
        if !self.chunks.contains_key(&pos) {
            let chunk = VoxelChunk::new(pos, &mut DefaultWorldGenerator);
            self.chunks.insert(pos, chunk);

            false
        } else {
            true
        }
    }

    /// Whether the chunk at the specified position is loaded. A chunk being loaded means that it is simulated and visible in the world.
    pub fn is_chunk_loaded(&self, pos: ChunkPos) -> bool {
        self.loaded_chunks.contains(&pos)
    }

    /// Marks a chunk as being loaded. Should be called after spawning the chunk's mesh.
    pub fn mark_chunk_loaded(&mut self, pos: ChunkPos) {
        self.loaded_chunks.insert(pos);
    }

    pub fn get_and_load_chunks_in_view_distance(
        &mut self,
        view_distance: &ViewDistance,
        player_chunk_pos: ChunkPos,
    ) {
        let range = -(view_distance.distance as i32)..(view_distance.distance as i32);

        let mut chunks = Vec::new();

        for x in -(view_distance.distance as i32)..(view_distance.distance as i32) {
            for y in -(view_distance.distance as i32)..(view_distance.distance as i32) {
                for z in -(view_distance.distance as i32)..(view_distance.distance as i32) {
                    let chunk_position = player_chunk_pos + ChunkPos::new(x, y, z);
                    self.create_chunk(chunk_position);

                    let chunk = self.get_chunk(chunk_position).unwrap();
                    chunks.push(chunk);
                }
            }
        }
    }

    pub fn get(&self, pos: WorldPos) -> Option<&Box<dyn Voxel>> {
        let chunk = self.chunks.get(&super::world_pos_to_chunk_pos(pos))?;
        chunk.try_get_at((pos.x as isize, pos.y as isize, pos.z as isize))
    }

    pub fn get_mut(&mut self, pos: WorldPos) -> Option<&mut Box<dyn Voxel>> {
        let chunk = self.chunks.get_mut(&super::world_pos_to_chunk_pos(pos))?;
        chunk.try_get_at_mut((pos.x as isize, pos.y as isize, pos.z as isize))
    }

    pub fn get_mesh(&mut self, pos: ChunkPos) -> Option<Mesh> {
        let chunk = self.chunks.get_mut(&pos)?;
        Some(chunk.get_or_create_mesh())
    }
}
