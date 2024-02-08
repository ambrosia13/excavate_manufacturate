use bevy::{
    prelude::*,
    tasks::{ComputeTaskPool, Task},
    utils::HashMap,
};
use crossbeam_queue::SegQueue;

use crate::{
    player::Player,
    util::{
        block_pos::BlockPos,
        chunk_pos::{ChunkPos, LocalChunkPos},
    },
};

use super::{
    chunk::ChunkOcclusionData, render_distance::RenderDistance,
    world_access::ExcavateManufacturateWorld,
};

/// The currently spawned chunks.
#[derive(Resource, Deref, DerefMut)]
pub struct SpawnedChunks(HashMap<ChunkPos, Entity>);

/// Handles to chunks whose meshes have already been created.
#[derive(Resource, Deref, DerefMut)]
pub struct ChunkMeshes(HashMap<ChunkPos, Handle<Mesh>>);

#[derive(Resource, Deref, DerefMut)]
pub struct ChunkSpawnQueue(SegQueue<ChunkPos>);

pub fn setup_chunk_data(mut commands: Commands) {
    commands.insert_resource(SpawnedChunks(HashMap::new()));
    commands.insert_resource(ChunkMeshes(HashMap::new()));
    commands.insert_resource(ChunkSpawnQueue(SegQueue::new()));

    info!("Initialized chunk data structures");
}

pub fn despawn_chunk_data(mut commands: Commands) {
    commands.remove_resource::<SpawnedChunks>();
    commands.remove_resource::<ChunkMeshes>();
    commands.remove_resource::<ChunkSpawnQueue>();

    info!("Removed chunk data structures");
}

pub fn populate_chunk_spawn_queue(
    player_query: Query<&ChunkPos, With<Player>>,
    chunk_spawn_queue: Res<ChunkSpawnQueue>,
    render_distance: Res<RenderDistance>,
    spawned_chunks: Res<SpawnedChunks>,
) {
    let player_chunk_pos = *player_query.single();

    let lower = -render_distance.chunks();
    let upper = render_distance.chunks();

    for x_offset in lower..=upper {
        for y_offset in lower..=upper {
            for z_offset in lower..=upper {
                let chunk_pos = player_chunk_pos + ChunkPos::new(x_offset, y_offset, z_offset);

                if !spawned_chunks.contains_key(&chunk_pos) {
                    chunk_spawn_queue.push(chunk_pos);
                }
            }
        }
    }
}

#[derive(Component)]
pub struct SpawnedChunkTask(Task<(ChunkPos, Mesh)>);

pub fn spawn_chunks(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // mut spawned_chunks: ResMut<SpawnedChunks>,
    em_world: Res<ExcavateManufacturateWorld>,
    chunk_spawn_queue: Res<ChunkSpawnQueue>,
) {
    let thread_pool = ComputeTaskPool::get();

    while let Some(chunk_pos) = chunk_spawn_queue.pop() {
        let Some(chunk) = em_world.get_chunk(chunk_pos) else {
            // We can't spawn an empty chunk
            continue;
        };

        let occlusion_data = ChunkOcclusionData::from_chunk(chunk);

        let task = thread_pool.spawn(async move { (chunk_pos, occlusion_data.get_mesh()) });
        commands.spawn(SpawnedChunkTask(task));

        // let mesh = chunk.get_mesh();

        // let entity = commands
        //     .spawn((
        //         MaterialMeshBundle {
        //             mesh: meshes.add(mesh),
        //             material: materials.add(StandardMaterial {
        //                 base_color: Color::RED,
        //                 ..Default::default()
        //             }),
        //             transform: Transform::from_translation(
        //                 BlockPos::from(chunk_pos).as_vec3() - 1.0,
        //             ),
        //             ..Default::default()
        //         },
        //         chunk_pos,
        //     ))
        //     .id();

        // if let Some(old_chunk) = spawned_chunks.insert(chunk_pos, entity) {
        //     commands.entity(old_chunk).despawn();
        // }
    }
}

pub fn poll_spawned_chunks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SpawnedChunkTask)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawned_chunks: ResMut<SpawnedChunks>,
) {
    for (task_entity, mut task) in tasks.iter_mut() {
        if let Some((chunk_pos, mesh)) =
            bevy::tasks::block_on(futures_lite::future::poll_once(&mut task.0))
        {
            let entity = commands
                .spawn((
                    MaterialMeshBundle {
                        mesh: meshes.add(mesh),
                        material: materials.add(StandardMaterial {
                            base_color: Color::RED,
                            ..Default::default()
                        }),
                        transform: Transform::from_translation(
                            BlockPos::from(chunk_pos).as_vec3() - 1.0,
                        ),
                        ..Default::default()
                    },
                    chunk_pos,
                ))
                .id();

            if let Some(old_chunk) = spawned_chunks.insert(chunk_pos, entity) {
                commands.entity(old_chunk).despawn();
            }

            commands.entity(task_entity).despawn();
        }
    }
}

pub fn despawn_chunks(
    mut commands: Commands,
    chunks_query: Query<&ChunkPos>,
    player_query: Query<&ChunkPos, With<Player>>,
    render_distance: Res<RenderDistance>,
    mut spawned_chunks: ResMut<SpawnedChunks>,
) {
    let player_chunk_pos = *player_query.single();

    for &chunk_pos in chunks_query.iter() {
        let local_chunk_pos = LocalChunkPos::from(chunk_pos, player_chunk_pos);

        if !render_distance.contains(local_chunk_pos) {
            if let Some(entity) = spawned_chunks.remove(&chunk_pos) {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn despawn_all_chunks(
    mut commands: Commands,
    chunks_query: Query<&ChunkPos>,
    mut spawned_chunks: ResMut<SpawnedChunks>,
) {
    for &chunk_pos in chunks_query.iter() {
        if let Some(entity) = spawned_chunks.remove(&chunk_pos) {
            commands.entity(entity).despawn();
        }
    }
}
