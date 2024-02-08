use bevy::prelude::*;

use crate::{
    state::GameState,
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

pub mod cursor;
pub mod interact;

pub struct ExavateManufacturatePlayerPlugin;

impl Plugin for ExavateManufacturatePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_player)
            .add_systems(OnExit(GameState::InGame), despawn_player)
            .add_systems(
                Update,
                (
                    update_player_pos,
                    interact::destroy_block,
                    cursor::draw_crosshair,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct Player;

fn setup_player(mut commands: Commands) {
    let player_block_pos = BlockPos::new(0, 30, 0);
    let player_chunk_pos = ChunkPos::from(player_block_pos);

    commands.spawn((
        Player,
        Camera3dBundle {
            transform: Transform::from_translation(player_block_pos.as_vec3()),
            ..Default::default()
        },
        player_chunk_pos,
        player_block_pos,
        bevy_flycam::FlyCam,
    ));

    info!("Initialized player camera");
}

fn update_player_pos(
    mut player_query: Query<(&mut ChunkPos, &mut BlockPos, &Transform), With<Player>>,
) {
    let (mut chunk_pos, mut block_pos, transform) = player_query.single_mut();
    *block_pos = BlockPos::from(transform.translation);
    *chunk_pos = ChunkPos::from(*block_pos);
}

fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let player = player_query.single();
    commands.entity(player).despawn();

    info!("Removed player camera");
}
