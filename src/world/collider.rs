use bevy::{prelude::*, utils::HashSet};
use bevy_rapier3d::geometry::{Collider, ColliderDisabled};

use crate::{
    mob::physics::MobVelocity,
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

use super::render::SpawnedChunks;

#[derive(Event)]
pub struct ChunkColliderEnableEvent(pub ChunkPos);

#[derive(Event)]
pub struct ChunkColliderDisableEvent(pub ChunkPos);

#[allow(clippy::type_complexity)]
pub fn send_enable_chunk_colliders_near_mobs(
    mut events: EventWriter<ChunkColliderEnableEvent>,
    mob_query: Query<&Transform, With<MobVelocity>>,
    collider_query: Query<Entity, (With<ChunkPos>, With<Collider>, Without<ColliderDisabled>)>,
    spawned_chunks: Res<SpawnedChunks>,
) {
    for transform in mob_query.iter() {
        let block_pos = BlockPos::from(transform.translation);

        let chunk_positions = block_pos.get_touched_chunk_positions();

        for chunk_pos in chunk_positions {
            if let Some(&chunk_entity) = spawned_chunks.get(&chunk_pos) {
                // If the chunk already has a collider, don't make a new one
                if collider_query.contains(chunk_entity) {
                    continue;
                }

                events.send(ChunkColliderEnableEvent(chunk_pos));
            }
        }
    }
}

pub fn send_disable_chunk_colliders_on_deserted_chunks(
    mut events: EventWriter<ChunkColliderDisableEvent>,
    mob_query: Query<&Transform, With<MobVelocity>>,
    query: Query<&ChunkPos, With<Collider>>,
) {
    let mut allowed_positions = HashSet::new();

    for transform in mob_query.iter() {
        let block_pos = BlockPos::from(transform.translation);
        allowed_positions.extend(block_pos.get_touched_chunk_positions());
    }

    for chunk_pos in query.iter() {
        if !allowed_positions.contains(chunk_pos) {
            events.send(ChunkColliderDisableEvent(*chunk_pos));
        }
    }
}

pub fn enable_chunk_colliders(
    mut commands: Commands,
    mut events: EventReader<ChunkColliderEnableEvent>,
    spawned_chunks: Res<SpawnedChunks>,
) {
    for ChunkColliderEnableEvent(chunk_pos) in events.read() {
        if let Some(mut entity_commands) = spawned_chunks
            .get(chunk_pos)
            .and_then(|&entity| commands.get_entity(entity))
        {
            entity_commands.remove::<ColliderDisabled>();
        }
    }
}

pub fn disable_chunk_colliders(
    mut commands: Commands,
    mut events: EventReader<ChunkColliderDisableEvent>,
    spawned_chunks: Res<SpawnedChunks>,
) {
    for ChunkColliderDisableEvent(chunk_pos) in events.read() {
        if let Some(mut entity_commands) = spawned_chunks
            .get(chunk_pos)
            .and_then(|&entity| commands.get_entity(entity))
        {
            entity_commands.insert(ColliderDisabled);
        }
    }
}
