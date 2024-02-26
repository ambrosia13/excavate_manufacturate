use bevy::prelude::*;

use crate::{
    util::{self, block_pos::BlockPos, raytrace::Hit},
    world::{
        block::{registry::BlockRegistry, static_block_data::BlockHardnessLevel, BlockData},
        render::ChunkSpawnQueue,
        world_access::ExcavateManufacturateWorld,
    },
};

use super::{keybinds::PlayerKeybinds, Player};

pub fn raycast(
    player_transform: Query<&Transform, With<Player>>,
    em_world: Res<ExcavateManufacturateWorld>,
) -> Option<Hit> {
    let player_transform = player_transform.get_single().ok()?;

    util::raytrace::raytrace_dda(
        player_transform.translation,
        Vec3::from(player_transform.forward()),
        30,
        em_world.hit_evaluator(),
    )
}

pub fn draw_crosshair(In(hit): In<Option<Hit>>, mut gizmos: Gizmos) -> Option<Hit> {
    if let Some(hit) = hit {
        gizmos.sphere(hit.position, Quat::IDENTITY, 0.25, Color::WHITE);

        Some(hit)
    } else {
        None
    }
}

pub fn handle_destroy_block(
    In(hit): In<Option<Hit>>,

    mut em_world: ResMut<ExcavateManufacturateWorld>,
    block_registry: Res<BlockRegistry>,

    chunk_spawn_queue: Res<ChunkSpawnQueue>,

    input: Res<ButtonInput<MouseButton>>,
    keybinds: Res<PlayerKeybinds>,
) -> Option<Hit> {
    if !input.just_pressed(keybinds.break_block) {
        None
    } else if let Some(hit) = hit {
        if input.just_pressed(keybinds.break_block) {
            let block_pos = BlockPos::from(hit.position - 0.1 * hit.normal);
            let block_data = em_world.get_block(block_pos);

            let block_can_be_destroyed = block_data.is_some_and(|block_data| {
                block_data.as_ref().is_some_and(|block_type| {
                    block_registry.get_block_data(block_type.id).hardness
                        != BlockHardnessLevel::Unbreakable
                })
            });

            if block_can_be_destroyed && em_world.set_block(block_pos, BlockData::None) {
                chunk_spawn_queue.submit_on_block_update(block_pos);
            }
        }

        Some(hit)
    } else {
        None
    }
}

pub fn finish_interaction(_input: In<Option<Hit>>) {}

pub fn destroy_block(
    player_transform: Query<&Transform, With<Player>>,
    input: Res<ButtonInput<MouseButton>>,
    keybinds: Res<PlayerKeybinds>,
    mut em_world: ResMut<ExcavateManufacturateWorld>,
    chunk_spawn_queue: Res<ChunkSpawnQueue>,
    mut gizmos: Gizmos,
    block_registry: Res<BlockRegistry>,
) {
    let player_transform = player_transform.single();

    if let Some(Hit { position, normal }) = util::raytrace::raytrace_dda(
        player_transform.translation,
        Vec3::from(player_transform.forward()),
        30,
        em_world.hit_evaluator(),
    ) {
        // Offset the position by the negative normal to avoid error
        let block_pos = BlockPos::from(position - 0.1 * normal);

        gizmos.sphere(position, Quat::IDENTITY, 0.25, Color::WHITE);

        let block_data = em_world.get_block(block_pos);

        let block_can_be_destroyed = block_data.is_some_and(|block_data| {
            block_data.as_ref().is_some_and(|block_type| {
                block_registry.get_block_data(block_type.id).hardness
                    != BlockHardnessLevel::Unbreakable
            })
        });

        if block_can_be_destroyed
            && input.just_pressed(keybinds.break_block)
            && em_world.set_block(block_pos, BlockData::None)
        {
            chunk_spawn_queue.submit_on_block_update(block_pos);
            info!(
                "Successfully removed block at {:?}. Player position is {:?}",
                block_pos,
                BlockPos::from(player_transform.translation)
            );
        }
    }
}
