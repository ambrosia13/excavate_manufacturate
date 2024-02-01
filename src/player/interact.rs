use bevy::{math::Vec3A, prelude::*, render::mesh::shape::Cube};

use crate::{
    util::{self, block_pos::BlockPos, chunk_pos::ChunkPos, dda::Hit},
    world::{block::BlockData, render::ChunkSpawnQueue, world_access::ExcavateManufacturateWorld},
};

use super::Player;

pub fn destroy_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_transform: Query<&Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    mut ev_world: ResMut<ExcavateManufacturateWorld>,
    chunk_spawn_queue: Res<ChunkSpawnQueue>,
) {
    if !input.just_pressed(KeyCode::B) {
        return;
    }

    let player_transform = player_transform.single();
    // let block_pos = BlockPos::from(player_transform.translation);

    // info!(
    //     "Player translation: {:?}, block pos: {:?}, block: {:?}",
    //     player_transform.translation,
    //     block_pos,
    //     ev_world.get_block(block_pos)
    // );

    // ev_world.set_block(block_pos, BlockData::Empty);
    // chunk_spawn_queue.push(ChunkPos::from(block_pos));

    if let Some(Hit { position, .. }) = util::dda::raytrace_dda(
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
        let block_pos = BlockPos::from(Vec3::from(position));

        if ev_world.set_block(block_pos, BlockData::Empty) {
            chunk_spawn_queue.push(ChunkPos::from(block_pos));
            info!(
                "Successfully removed block at {:?}. Player position is {:?}",
                block_pos,
                BlockPos::from(player_transform.translation)
            );
        }

        info!("Hit position is {:?}", block_pos);
    } else {
        info!("No hit detected");
    }
}
