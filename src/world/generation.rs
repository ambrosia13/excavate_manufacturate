use std::sync::Arc;

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    utils::HashSet,
};

use crate::{
    player::Player,
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
    world::block,
};

use super::{
    block::BlockData,
    chunk::ChunkData,
    render::{ChunkSpawnQueue, SpawnedChunks},
    render_distance::RenderDistance,
    world_access::ExcavateManufacturateWorldAccess,
};
use crate::world::world_access::ExcavateManufacturateWorld;

#[derive(Resource, Deref, DerefMut)]
pub struct WorldGeneratorResource(Arc<WorldGenerator>);

pub struct WorldGenerator {
    pub terrain_noise: fn(BlockPos) -> BlockData,
    pub landscape_feature_generator: fn(BlockPos, &mut ExcavateManufacturateWorld),
}

impl WorldGenerator {
    pub fn generate_terrain_noise(&self, block_pos: BlockPos) -> BlockData {
        (self.terrain_noise)(block_pos)
    }
}

pub fn setup_world_generator(mut commands: Commands) {
    let world_generator = WorldGenerator {
        terrain_noise: |block_pos| {
            use block::excavatemanufacturate_blocks::block_types::*;

            match block_pos.y.cmp(&0) {
                std::cmp::Ordering::Less => BlockData::None,
                std::cmp::Ordering::Equal => BlockData::Some(BEDROCK),
                std::cmp::Ordering::Greater => {
                    let position = block_pos.as_vec3();

                    let hills_generator = |position: Vec3| {
                        position.y + 10.0 * noisy_bevy::simplex_noise_2d(position.xz() * 0.025)
                            < 20.0
                    };

                    if hills_generator(position) {
                        if hills_generator(position + Vec3::Y) {
                            // Grass is above this block
                            BlockData::Some(DIRT)
                        } else {
                            BlockData::Some(GRASS)
                        }
                    } else {
                        BlockData::None
                    }
                }
            }
        },
        landscape_feature_generator: |_, _| {},
    };

    commands.insert_resource(WorldGeneratorResource(Arc::new(world_generator)));
    info!("Initialized world generator");
}

pub fn remove_world_generator(mut commands: Commands) {
    commands.remove_resource::<WorldGeneratorResource>();
    info!("Removed world generator");
}

/// Singlethreaded chunk generation system. Currently unused, but is kept in code for comparison purposes.
#[allow(unused)]
pub fn generate_chunks(
    em_world: Res<ExcavateManufacturateWorldAccess>,
    world_generator: Res<WorldGeneratorResource>,
    render_distance: Res<RenderDistance>,
    player_transform: Query<&Transform, With<Player>>,
) {
    let mut em_world = em_world.lock().unwrap();

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

                let chunk_data = ChunkData::with_data(1, |block_pos| {
                    let block_pos = block_pos + BlockPos::from(chunk_pos);
                    world_generator.generate_terrain_noise(block_pos)
                });

                em_world.insert_chunk(chunk_pos, chunk_data);
            }
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct PossiblyGeneratedChunks(HashSet<ChunkPos>);

pub fn setup_chunk_generation_structures(mut commands: Commands) {
    commands.insert_resource(PossiblyGeneratedChunks(HashSet::new()));
    info!("Initialized chunk generation data structures");
}

pub fn remove_chunk_generation_structures(mut commands: Commands) {
    commands.remove_resource::<PossiblyGeneratedChunks>();
    info!("Removed chunk generation data structures");
}

#[derive(Component)]
pub struct GeneratedChunkTask(Task<(ChunkPos, ChunkData)>);

pub fn generate_chunks_on_thread_pool(
    mut commands: Commands,
    em_world: Res<ExcavateManufacturateWorldAccess>,
    world_generator: Res<WorldGeneratorResource>,
    render_distance: Res<RenderDistance>,
    player_query: Query<&ChunkPos, With<Player>>,
    mut possibly_generated_chunks: ResMut<PossiblyGeneratedChunks>,
) {
    let em_world = em_world.lock().unwrap();
    let player_chunk_pos = *player_query.single();

    let lower = -render_distance.chunks();
    let upper = render_distance.chunks();

    let thread_pool = AsyncComputeTaskPool::get();

    for x_offset in lower..=upper {
        for y_offset in lower..=upper {
            for z_offset in lower..=upper {
                let chunk_pos = player_chunk_pos + ChunkPos::new(x_offset, y_offset, z_offset);

                // If the chunk has already been set for generation, or has already been generated, skip this
                if possibly_generated_chunks.contains(&chunk_pos)
                    || em_world.chunk_exists(chunk_pos)
                {
                    continue;
                }

                let world_generator = world_generator.clone();

                let task = thread_pool.spawn(async move {
                    let chunk_data = ChunkData::with_data(1, |block_pos| {
                        let block_pos = block_pos + BlockPos::from(chunk_pos);
                        world_generator.generate_terrain_noise(block_pos)
                    });

                    (chunk_pos, chunk_data)
                });

                commands.spawn(GeneratedChunkTask(task));
                possibly_generated_chunks.insert(chunk_pos); // mark this chunk as being generated
            }
        }
    }
}

pub fn poll_generated_chunks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut GeneratedChunkTask)>,
    em_world: Res<ExcavateManufacturateWorldAccess>,
    spawned_chunks: Res<SpawnedChunks>,
    spawn_queue: Res<ChunkSpawnQueue>,
) {
    let mut em_world = em_world.lock().unwrap();

    for (entity, mut task) in tasks.iter_mut() {
        if let Some((chunk_pos, chunk_data)) =
            bevy::tasks::block_on(futures_lite::future::poll_once(&mut task.0))
        {
            em_world.insert_chunk(chunk_pos, chunk_data);
            commands.entity(entity).despawn();

            let chunk_positions_to_rebuild = [
                chunk_pos,
                chunk_pos + ChunkPos::new(1, 0, 0),
                chunk_pos - ChunkPos::new(1, 0, 0),
                chunk_pos + ChunkPos::new(0, 1, 0),
                chunk_pos - ChunkPos::new(0, 1, 0),
                chunk_pos + ChunkPos::new(0, 0, 1),
                chunk_pos - ChunkPos::new(0, 0, 1),
            ];

            // If the chunk was falsely spawned before it finished generating, rebuild it.
            // Also, rebuild its neighbors, because their own mesh will need to be updated based on the newly generated chunk.
            for chunk_pos in chunk_positions_to_rebuild {
                if spawned_chunks.contains_key(&chunk_pos) {
                    spawn_queue.push(chunk_pos);
                }
            }
        }
    }
}
