use bevy::{prelude::*, utils::HashMap};

use crate::util::{block_pos::BlockPos, chunk_pos::ChunkPos};

use super::{block::BlockData, chunk::ChunkData, render::ChunkSpawnQueue};

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

    pub fn total_chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn get_block(&self, block_pos: BlockPos) -> Option<&BlockData> {
        if let Some(chunk_data) = self.get_chunk(ChunkPos::from(block_pos)) {
            Some(chunk_data.get(block_pos))
        } else {
            None
        }
    }

    /// Attempts to set the block at the position. If the block is in a chunk that does not exist and the block is
    /// not empty, creates a new chunk at that position with just the block inside.
    pub fn set_block(&mut self, block_pos: BlockPos, block_data: BlockData) -> bool {
        if let Some(chunk_data) = self.get_chunk_mut(ChunkPos::from(block_pos)) {
            chunk_data.set(block_pos, block_data);
            true
        } else if block_data.is_some() {
            let chunk_pos = ChunkPos::from(block_pos);

            let mut chunk_data = ChunkData::empty();
            chunk_data.set(block_pos, block_data);

            self.insert_chunk(chunk_pos, chunk_data);

            true
        } else {
            false
        }
    }

    pub fn hit_evaluator(&self) -> impl Fn(IVec3) -> bool + '_ {
        |pos: IVec3| {
            let block_pos = BlockPos::from(pos);
            self.get_block(block_pos)
                .is_some_and(|block_data| block_data.is_some())
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ExcavateManufacturateWorld::new());
    info!("Set up world data");
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<ExcavateManufacturateWorld>();
    info!("Cleaned up world data");
}

#[derive(Event)]
pub struct BlockPlaceEvent {
    pub pos: BlockPos,
    pub block: BlockData,
}

#[derive(Event)]
pub struct BlockDestroyEvent {
    pub pos: BlockPos,
    pub previous_block: BlockData,
}

impl BlockDestroyEvent {
    pub fn create(block_pos: BlockPos, world: &ExcavateManufacturateWorld) -> Self {
        Self {
            pos: block_pos,
            previous_block: world.get_block(block_pos).unwrap().clone(),
        }
    }
}

pub fn apply_block_place_events(
    mut events: ResMut<Events<BlockPlaceEvent>>,
    mut em_world: ResMut<ExcavateManufacturateWorld>,
) {
    for event in events.drain() {
        em_world.set_block(event.pos, event.block);
    }
}

pub fn apply_block_destroy_events(
    mut commands: Commands,
    mut events: ResMut<Events<BlockDestroyEvent>>,
    mut em_world: ResMut<ExcavateManufacturateWorld>,
    chunk_spawn_queue: Res<ChunkSpawnQueue>,
) {
    for event in events.drain() {
        if let Some(entity) = event
            .previous_block
            .as_ref()
            .and_then(|block| block.dynamic_data)
        {
            // There was dynamic data attached to this block, despawn it.
            commands.entity(entity).despawn();
        }

        if em_world.set_block(event.pos, BlockData::none()) {
            // Redraw chunks if needed
            chunk_spawn_queue.submit_on_block_update(event.pos);
        }
    }
}
