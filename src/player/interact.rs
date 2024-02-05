use bevy::prelude::*;

use crate::{
    util::{self, block_pos::BlockPos, chunk_pos::ChunkPos, raytrace::Hit},
    world::{block::BlockData, render::ChunkSpawnQueue, world_access::ExcavateManufacturateWorld},
};

use super::Player;

pub fn destroy_block(
    player_transform: Query<&Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    mut ev_world: ResMut<ExcavateManufacturateWorld>,
    chunk_spawn_queue: Res<ChunkSpawnQueue>,
    mut gizmos: Gizmos,
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
                .is_some_and(|block_data| match block_data {
                    BlockData::Empty => false,
                    BlockData::Full(_) => true,
                })
        },
    ) {
        // Offset the position by the negative normal to avoid error
        let block_pos = BlockPos::from(Vec3::from(position + 0.1 * normal));

        gizmos.sphere(Vec3::from(position), Quat::IDENTITY, 0.25, Color::WHITE);

        if input.just_pressed(KeyCode::B) && ev_world.set_block(block_pos, BlockData::Empty) {
            chunk_spawn_queue.push(ChunkPos::from(block_pos));
            info!(
                "Successfully removed block at {:?}. Player position is {:?}",
                block_pos,
                BlockPos::from(player_transform.translation)
            );
        }
    }
}
