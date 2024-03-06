use std::ops::Deref;

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, ComputeTaskPool, Task, TaskPool},
    utils::HashSet,
};

use crate::{
    mob::player::Player,
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

use super::{
    chunk::ChunkData,
    render::{ChunkSpawnQueue, SpawnedChunks},
    render_distance::RenderDistance,
    worldgen::WorldGeneratorResource,
};
use crate::world::world_access::ExcavateManufacturateWorld;

#[derive(Resource, Deref, DerefMut)]
pub struct PossiblyGeneratedChunks(HashSet<ChunkPos>);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(PossiblyGeneratedChunks(HashSet::new()));
    info!("Setup chunk generator");
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<PossiblyGeneratedChunks>();
    info!("Cleaned up chunk generator");
}

#[derive(Component)]
pub struct GeneratedChunkTask(Task<(ChunkPos, ChunkData)>);

// So we can make which task pool you use be generic
pub trait GetTaskPool: Deref<Target = TaskPool> + 'static {
    fn get() -> &'static Self;
}

impl GetTaskPool for AsyncComputeTaskPool {
    #[inline(always)]
    fn get() -> &'static Self {
        AsyncComputeTaskPool::get()
    }
}

impl GetTaskPool for ComputeTaskPool {
    #[inline(always)]
    fn get() -> &'static Self {
        ComputeTaskPool::get()
    }
}
pub fn generate_chunks_multithreaded<T: GetTaskPool>(
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

    let thread_pool = T::get();

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
            let chunk_data = ChunkData::with_data(|block_pos| {
                let block_pos = block_pos + BlockPos::from(chunk_pos);
                world_generator.generate_terrain_noise(block_pos)
            });

            (chunk_pos, chunk_data)
        });

        commands.spawn(GeneratedChunkTask(task));
        possibly_generated_chunks.insert(chunk_pos); // mark this chunk as being generated
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
