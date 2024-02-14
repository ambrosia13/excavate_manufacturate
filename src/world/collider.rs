use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;

use crate::{
    player::Player,
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

use super::{
    render::{ChunkMeshes, SpawnedChunks},
    CHUNK_SIZE_INT,
};

pub fn insert_collider_on_player_chunk_pos(
    mut commands: Commands,
    player_query: Query<(&ChunkPos, &BlockPos), With<Player>>,
    spawned_chunks: Res<SpawnedChunks>,
    chunk_meshes: Res<ChunkMeshes>,
    meshes: Res<Assets<Mesh>>,
) {
    let (&player_chunk_pos, &player_block_pos) = player_query.single();

    let mut chunk_positions = vec![player_chunk_pos];

    let on_chunk_borders_neg = player_block_pos.cmpeq(IVec3::splat(0));
    let on_chunk_borders_pos = player_block_pos.cmpeq(IVec3::splat(CHUNK_SIZE_INT - 1));

    if on_chunk_borders_pos.x {
        chunk_positions.push(player_chunk_pos + ChunkPos::new(1, 0, 0));
    }
    if on_chunk_borders_neg.x {
        chunk_positions.push(player_chunk_pos - ChunkPos::new(1, 0, 0));
    }
    if on_chunk_borders_pos.y {
        chunk_positions.push(player_chunk_pos + ChunkPos::new(0, 1, 0));
    }
    if on_chunk_borders_neg.y {
        chunk_positions.push(player_chunk_pos - ChunkPos::new(0, 1, 0));
    }
    if on_chunk_borders_pos.z {
        chunk_positions.push(player_chunk_pos + ChunkPos::new(0, 0, 1));
    }
    if on_chunk_borders_neg.z {
        chunk_positions.push(player_chunk_pos - ChunkPos::new(0, 0, 1));
    }

    for chunk_pos in chunk_positions {
        if let Some(collider) = chunk_meshes
            .get(&chunk_pos)
            .and_then(|handle| meshes.get(handle))
            .and_then(|mesh| {
                Collider::from_bevy_mesh(
                    mesh,
                    &bevy_rapier3d::geometry::ComputedColliderShape::TriMesh,
                )
            })
        {
            if let Some(entity) = spawned_chunks.get(&chunk_pos) {
                if let Some(mut entity_commands) = commands.get_entity(*entity) {
                    entity_commands.insert(collider);
                } else {
                    warn!("Entity at chunk position {:?} doesn't exist", chunk_pos);
                }
            } else {
                warn!(
                    "Chunk at player chunk position {:?} doesn't exist",
                    chunk_pos
                );
            }
        }
    }
}

pub fn remove_collider_on_faraway_chunks(
    mut commands: Commands,
    player_query: Query<(&ChunkPos, &BlockPos), With<Player>>,
    query: Query<(Entity, &ChunkPos), With<Collider>>,
) {
    let (&player_chunk_pos, &player_block_pos) = player_query.single();

    let mut allowed_positions = vec![player_chunk_pos];

    let on_chunk_borders_neg = player_block_pos.cmpeq(IVec3::splat(0));
    let on_chunk_borders_pos = player_block_pos.cmpeq(IVec3::splat(CHUNK_SIZE_INT - 1));

    if on_chunk_borders_pos.x {
        allowed_positions.push(player_chunk_pos + ChunkPos::new(1, 0, 0));
    }
    if on_chunk_borders_neg.x {
        allowed_positions.push(player_chunk_pos - ChunkPos::new(1, 0, 0));
    }
    if on_chunk_borders_pos.y {
        allowed_positions.push(player_chunk_pos + ChunkPos::new(0, 1, 0));
    }
    if on_chunk_borders_neg.y {
        allowed_positions.push(player_chunk_pos - ChunkPos::new(0, 1, 0));
    }
    if on_chunk_borders_pos.z {
        allowed_positions.push(player_chunk_pos + ChunkPos::new(0, 0, 1));
    }
    if on_chunk_borders_neg.z {
        allowed_positions.push(player_chunk_pos - ChunkPos::new(0, 0, 1));
    }

    for (entity, &chunk_pos) in query.iter() {
        if !allowed_positions.contains(&chunk_pos) {
            commands.entity(entity).remove::<Collider>();
        }
    }
}
