use bevy::prelude::*;

use crate::state::GameState;

pub mod block;
pub mod chunk;
pub mod generation;
pub mod render;
pub mod render_distance;
pub mod world_access;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SIZE_PADDED: usize = CHUNK_SIZE + 2;
pub const CHUNK_SIZE_INT: i32 = CHUNK_SIZE as i32;

pub struct ExcavateManufacturateWorldPlugin;

impl Plugin for ExcavateManufacturateWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_distance::setup_render_distance)
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    world_access::setup_world_access,
                    generation::setup_world_generator,
                    render::setup_chunk_data,
                ),
            )
            .add_systems(
                Update,
                (
                    (
                        // Multithreaded chunk generation
                        generation::poll_generated_chunks,
                        generation::generate_chunks_on_thread_pool,
                    )
                        .chain(),
                    render::populate_chunk_spawn_queue,
                    render::spawn_chunks,
                    render::despawn_chunks,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                OnExit(GameState::InGame),
                (
                    world_access::despawn_world_access,
                    generation::despawn_world_generator,
                    render::despawn_chunk_data,
                    render::despawn_all_chunks,
                ),
            );
    }
}
