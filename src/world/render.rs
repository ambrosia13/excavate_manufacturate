use bevy::{prelude::*, utils::HashMap};
use crossbeam_queue::SegQueue;

use crate::{
    player::Player,
    util::{
        block_pos::BlockPos,
        chunk_pos::{ChunkPos, LocalChunkPos},
    },
};

use super::{
    block::registry::{BlockRegistry, TextureAtlasHandle},
    render_distance::RenderDistance,
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

impl ChunkSpawnQueue {
    pub fn submit_on_block_update(&self, block_pos: BlockPos) {
        block_pos
            .get_touched_chunk_positions()
            .into_iter()
            .for_each(|pos| self.push(pos));
    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(SpawnedChunks(HashMap::new()));
    commands.insert_resource(ChunkMeshes(HashMap::new()));
    commands.insert_resource(ChunkSpawnQueue(SegQueue::new()));

    info!("Setup chunk renderer");
}

pub fn cleanup(
    mut commands: Commands,
    chunks_query: Query<&ChunkPos>,
    spawned_chunks: ResMut<SpawnedChunks>,
) {
    commands.remove_resource::<SpawnedChunks>();
    commands.remove_resource::<ChunkMeshes>();
    commands.remove_resource::<ChunkSpawnQueue>();

    despawn_all_chunks(commands, chunks_query, spawned_chunks);

    info!("Cleaned up chunk renderer");
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

#[allow(clippy::too_many_arguments)]
pub fn spawn_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawned_chunks: ResMut<SpawnedChunks>,
    mut chunk_meshes: ResMut<ChunkMeshes>,
    em_world: Res<ExcavateManufacturateWorld>,
    chunk_spawn_queue: Res<ChunkSpawnQueue>,
    block_registry: Res<BlockRegistry>,
    texture_atlas_handle: Res<TextureAtlasHandle>,
) {
    while let Some(chunk_pos) = chunk_spawn_queue.pop() {
        let Some(chunk) = em_world.get_chunk(chunk_pos) else {
            // The chunk doesn't exist in the world for some reason
            continue;
        };

        if chunk.is_empty() {
            // Don't spawn a chunk with no data in it
            continue;
        }

        // let mesh_handle = if let Some(mesh_handle) = chunk_meshes.get(&chunk_pos) {
        //     // The mesh already exists, use a weak handle to it to spawn it
        //     mesh_handle.clone_weak()
        // } else {
        //     // The mesh hasn't been created yet, let's create it
        //     let mesh = chunk.get_mesh(chunk_pos, &block_registry, &em_world);
        //     let mesh_handle = meshes.add(mesh);

        //     chunk_meshes.insert(chunk_pos, mesh_handle.clone());

        //     mesh_handle
        // };

        let mesh = chunk.get_mesh(chunk_pos, &block_registry, &em_world);
        let mesh_handle = meshes.add(mesh);

        chunk_meshes.insert(chunk_pos, mesh_handle.clone_weak());

        let entity = commands
            .spawn((
                MaterialMeshBundle {
                    mesh: mesh_handle,
                    material: materials.add(StandardMaterial {
                        base_color: Color::WHITE,
                        base_color_texture: Some(texture_atlas_handle.clone_weak()),
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
