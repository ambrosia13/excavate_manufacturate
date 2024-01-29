use std::sync::Arc;

use bevy::{
    prelude::*,
    tasks::{ComputeTaskPool, Task},
};

use crate::{
    player::Player,
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

use super::{
    block::{BlockData, BlockType},
    chunk::ChunkData,
    render::{ChunkSpawnQueue, SpawnedChunks},
    render_distance::RenderDistance,
};
use crate::world::world_access::ExcavateManufacturateWorld;

#[derive(Resource, Deref, DerefMut)]
pub struct WorldGeneratorResource(Arc<WorldGenerator>);

pub struct WorldGenerator {
    terrain_generator: fn(BlockPos) -> BlockData,
}

impl WorldGenerator {
    pub fn new(terrain_generator: fn(BlockPos) -> BlockData) -> Self {
        Self { terrain_generator }
    }

    pub fn generate_terrain(&self, block_pos: BlockPos) -> BlockData {
        (self.terrain_generator)(block_pos)
    }
}

pub fn setup_world_generator(mut commands: Commands) {
    let world_generator = WorldGenerator::new(|block_pos| {
        let position = block_pos.as_vec3();

        if position.y + 10.0 * noisy_bevy::simplex_noise_2d(position.xz() * 0.025) < 20.0 {
            BlockData::Full(BlockType::Debug)
        } else {
            BlockData::Empty
        }
    });

    commands.insert_resource(WorldGeneratorResource(Arc::new(world_generator)));
    info!("Initialized world generator");
}

pub fn despawn_world_generator(mut commands: Commands) {
    commands.remove_resource::<WorldGeneratorResource>();
    info!("Removed world generator");
}

/// Singlethreaded chunk generation system. Currently unused, but is kept in code for comparison purposes.
#[allow(unused)]
pub fn generate_chunks(
    mut em_world: ResMut<ExcavateManufacturateWorld>,
    world_generator: Res<WorldGeneratorResource>,
    render_distance: Res<RenderDistance>,
    player_transform: Query<&Transform, With<Player>>,
) {
    let player_translation = player_transform.single().translation;
    let player_chunk_pos = ChunkPos::from(BlockPos::from(player_translation));

    let lower = -render_distance.chunks();
    let upper = render_distance.chunks();

    for x_offset in lower..=upper {
        for y_offset in lower..=upper {
            for z_offset in lower..=upper {
                let chunk_pos = player_chunk_pos + ChunkPos::new(x_offset, y_offset, z_offset);

                // Don't generate a chunk that already exists
                if em_world.chunk_exists(chunk_pos) {
                    continue;
                }

                let chunk_data = ChunkData::with_data(|block_pos| {
                    let block_pos = block_pos + BlockPos::from(chunk_pos);
                    world_generator.generate_terrain(block_pos)
                });

                em_world.insert_chunk(chunk_pos, chunk_data);
            }
        }
    }
}

#[derive(Component)]
pub struct GeneratedChunkTask(Task<(ChunkPos, ChunkData)>);

pub fn generate_chunks_on_thread_pool(
    mut commands: Commands,
    em_world: Res<ExcavateManufacturateWorld>,
    world_generator: Res<WorldGeneratorResource>,
    render_distance: Res<RenderDistance>,
    player_transform: Query<&Transform, With<Player>>,
) {
    let player_translation = player_transform.single().translation;
    let player_chunk_pos = ChunkPos::from(BlockPos::from(player_translation));

    let lower = -render_distance.chunks();
    let upper = render_distance.chunks();

    let thread_pool = ComputeTaskPool::get();

    for x_offset in lower..=upper {
        for y_offset in lower..=upper {
            for z_offset in lower..=upper {
                let chunk_pos = player_chunk_pos + ChunkPos::new(x_offset, y_offset, z_offset);

                // Don't generate a chunk that already exists
                if em_world.chunk_exists(chunk_pos) {
                    continue;
                }

                let world_generator = world_generator.clone();

                let task = thread_pool.spawn(async move {
                    let chunk_data = ChunkData::with_data(|block_pos| {
                        let block_pos = block_pos + BlockPos::from(chunk_pos);
                        world_generator.generate_terrain(block_pos)
                    });

                    (chunk_pos, chunk_data)
                });

                commands.spawn(GeneratedChunkTask(task));
            }
        }
    }
}

pub fn poll_generated_chunks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut GeneratedChunkTask)>,
    mut em_world: ResMut<ExcavateManufacturateWorld>,
    spawned_chunks: Res<SpawnedChunks>,
    spawn_queue: Res<ChunkSpawnQueue>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some((chunk_pos, chunk_data)) =
            bevy::tasks::block_on(futures_lite::future::poll_once(&mut task.0))
        {
            em_world.insert_chunk(chunk_pos, chunk_data);
            commands.entity(entity).despawn();

            if spawned_chunks.contains_key(&chunk_pos) {
                // If the chunk was falsely spawned before it finished generating, rebuild it.
                spawn_queue.push(chunk_pos);
            }
        }
    }
}
