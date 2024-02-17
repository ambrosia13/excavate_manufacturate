use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

use crate::{
    player::Player,
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

use super::render::{ChunkMeshes, SpawnedChunks};

pub fn insert_collider_on_player_chunk_pos(
    mut commands: Commands,
    player_query: Query<&BlockPos, With<Player>>,
    collider_query: Query<Entity, (With<ChunkPos>, With<Collider>)>,
    spawned_chunks: Res<SpawnedChunks>,
    chunk_meshes: Res<ChunkMeshes>,
    meshes: Res<Assets<Mesh>>,
) {
    let player_block_pos = *player_query.single();

    let chunk_positions = player_block_pos.get_touched_chunk_positions();

    for chunk_pos in chunk_positions {
        if let Some(&chunk_entity) = spawned_chunks.get(&chunk_pos) {
            // If the chunk already has a collider, don't make a new one
            if collider_query.contains(chunk_entity) {
                continue;
            }
        }

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
                    entity_commands.insert((collider, RigidBody::Fixed));
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
    player_query: Query<&BlockPos, With<Player>>,
    query: Query<(Entity, &ChunkPos), With<Collider>>,
) {
    let player_block_pos = *player_query.single();

    let allowed_positions = player_block_pos.get_touched_chunk_positions();

    for (entity, &chunk_pos) in query.iter() {
        if !allowed_positions.contains(&chunk_pos) {
            commands.entity(entity).remove::<Collider>();
        }
    }
}
