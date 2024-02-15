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

                    let hills_multiplier =
                        noisy_bevy::simplex_noise_2d(position.xz() * 0.005 + 10000.0) * 0.5 + 0.5;

                    let hills_generator = |position: Vec3| {
                        position.y
                            + hills_multiplier
                                * 20.0
                                * noisy_bevy::simplex_noise_2d(position.xz() * 0.025)
                    };

                    let noise = hills_generator(position);

                    let base_ground_level = 30.0;

                    if noise < base_ground_level - 10.0 {
                        BlockData::Some(STONE)
                    } else if noise < base_ground_level - 1.0 {
                        BlockData::Some(DIRT)
                    } else if noise < base_ground_level {
                        BlockData::Some(GRASS)
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

                let chunk_data = ChunkData::with_data(1, |block_pos| {
                    let block_pos = block_pos + BlockPos::from(chunk_pos);
                    world_generator.generate_terrain_noise(block_pos)
                });

                // Note: because PossiblyGeneratedChunks is not used, this will cause lag because
                // the game is trying to generate the same chunks over and over
                if chunk_data.is_empty() {
                    drop(chunk_data);
                    continue;
                }

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
    em_world: Res<ExcavateManufacturateWorld>,
    world_generator: Res<WorldGeneratorResource>,
    render_distance: Res<RenderDistance>,
    player_query: Query<&ChunkPos, With<Player>>,
    mut possibly_generated_chunks: ResMut<PossiblyGeneratedChunks>,
) {
    let player_chunk_pos = *player_query.single();

    let lower = -render_distance.chunks();
    let upper = render_distance.chunks();

    let thread_pool = AsyncComputeTaskPool::get();

    let mut chunk_positions = Vec::new();

    for x_offset in lower..=upper {
        for y_offset in lower..=upper {
            for z_offset in lower..=upper {
                let chunk_pos = player_chunk_pos + ChunkPos::new(x_offset, y_offset, z_offset);

                if possibly_generated_chunks.contains(&chunk_pos)
                    || em_world.chunk_exists(chunk_pos)
                {
                    continue;
                }

                chunk_positions.push(chunk_pos);
            }
        }
    }

    chunk_positions.sort_unstable_by(|&a, &b| {
        player_chunk_pos
            .distance_squared(a.inner())
            .cmp(&player_chunk_pos.distance_squared(b.inner()))
    });

    for chunk_pos in chunk_positions {
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

    // for x_offset in lower..=upper {
    //     for y_offset in lower..=upper {
    //         for z_offset in lower..=upper {
    //             let chunk_pos = player_chunk_pos + ChunkPos::new(x_offset, y_offset, z_offset);

    //             // If the chunk has already been set for generation, or has already been generated, skip this
    //             if possibly_generated_chunks.contains(&chunk_pos)
    //                 || em_world.chunk_exists(chunk_pos)
    //             {
    //                 continue;
    //             }

    //             let world_generator = world_generator.clone();

    //             let task = thread_pool.spawn(async move {
    //                 let chunk_data = ChunkData::with_data(1, |block_pos| {
    //                     let block_pos = block_pos + BlockPos::from(chunk_pos);
    //                     world_generator.generate_terrain_noise(block_pos)
    //                 });

    //                 (chunk_pos, chunk_data)
    //             });

    //             commands.spawn(GeneratedChunkTask(task));
    //             possibly_generated_chunks.insert(chunk_pos); // mark this chunk as being generated
    //         }
    //     }
    // }
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
            if chunk_data.is_empty() {
                // Don't bother doing operations for an empty chunk, end this task.
                drop(chunk_data);
                commands.entity(entity).despawn();

                continue;
            }

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

pub fn debug_num_chunks_in_world(
    input: Res<Input<KeyCode>>,
    em_world: Res<ExcavateManufacturateWorld>,
) {
    if !input.just_pressed(KeyCode::Backslash) {
        return;
    }

    info!(
        "Number of chunks stored in the world: {}",
        em_world.total_chunk_count()
    );
}
