use bevy::prelude::*;

use crate::{
    util::{self, block_pos::BlockPos, chunk_pos::ChunkPos, raytrace::Hit},
    world::{
        block::{registry::BlockRegistry, static_block_data::BlockHardnessLevel, BlockData},
        render::ChunkSpawnQueue,
        world_access::ExcavateManufacturateWorld,
    },
};

use super::Player;

pub fn destroy_block(
    player_transform: Query<&Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    mut ev_world: ResMut<ExcavateManufacturateWorld>,
    chunk_spawn_queue: Res<ChunkSpawnQueue>,
    mut gizmos: Gizmos,
    block_registry: Res<BlockRegistry>,
) {
    let player_transform = player_transform.single();

    if let Some(Hit { position, normal }) = util::raytrace::raytrace_dda(
        player_transform.translation.into(),
        (player_transform.forward()).into(),
        30,
        |pos| {
            let block_pos = BlockPos::from(pos);
            ev_world
                .get_block(block_pos)
                .is_some_and(|block_data| block_data.is_some())
        },
    ) {
        // Offset the position by the negative normal to avoid error
        let mut block_pos = BlockPos::from((position - 0.1 * normal).as_ivec3());

        if position.x.is_sign_positive() {
            block_pos.x += 1;
        }
        if position.y.is_sign_positive() {
            block_pos.y += 1;
        }
        if position.z.is_sign_positive() {
            block_pos.z += 1;
        }

        gizmos.sphere(Vec3::from(position), Quat::IDENTITY, 0.25, Color::WHITE);

        let block_data = ev_world.get_block(block_pos);

        let block_can_be_destroyed = block_data.is_some_and(|block_data| {
            block_data.as_ref().is_some_and(|block_type| {
                block_registry.get_block_data(block_type.id).hardness
                    != BlockHardnessLevel::Unbreakable
            })
        });

        if block_can_be_destroyed
            && input.just_pressed(KeyCode::B)
            && ev_world.set_block(block_pos, BlockData::None)
        {
            chunk_spawn_queue.push(ChunkPos::from(block_pos));
            info!(
                "Successfully removed block at {:?}. Player position is {:?}",
                block_pos,
                BlockPos::from(player_transform.translation)
            );
        }
    }
}
