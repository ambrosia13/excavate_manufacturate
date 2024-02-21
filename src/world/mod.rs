use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    tasks::AsyncComputeTaskPool,
};

use crate::state::GameState;

pub mod block;
pub mod chunk;
pub mod collider;
pub mod generation;
pub mod render;
pub mod render_distance;
pub mod world_access;
pub mod worldgen;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_SIZE_INT: i32 = CHUNK_SIZE as i32;

pub const CHUNKS_RENDERED_PER_FRAME: usize = 5;

pub struct ExcavateManufacturateWorldPlugin;

impl Plugin for ExcavateManufacturateWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                render_distance::setup_render_distance,
                block::registry::setup_block_registry,
            ),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (
                setup_light,
                world_access::setup,
                worldgen::setup,
                generation::setup,
                render::setup,
            ),
        )
        .add_systems(
            Update,
            (
                (
                    // Multithreaded chunk generation
                    generation::poll_generated_chunks,
                    generation::generate_chunks_multithreaded::<AsyncComputeTaskPool>,
                ),
                // generation::generate_chunks,
                render::populate_chunk_spawn_queue,
                (
                    (render::spawn_chunks, render::despawn_chunks),
                    (
                        collider::insert_collider_on_player_chunk_pos,
                        collider::remove_collider_on_faraway_chunks,
                    ),
                )
                    .chain(),
                generation::debug_num_chunks_in_world,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            OnExit(GameState::InGame),
            (
                remove_light,
                world_access::cleanup,
                worldgen::cleanup,
                generation::cleanup,
                render::cleanup,
                render::despawn_all_chunks,
            ),
        );
    }
}

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    commands.insert_resource(DirectionalLightShadowMap { size: 1024 });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 32000.0,
            shadows_enabled: false,
            ..Default::default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.01,
            maximum_distance: 32.0 * CHUNK_SIZE as f32,
            first_cascade_far_bound: 10.0,
            overlap_proportion: 0.01,
        }
        .into(),
        transform: Transform::from_xyz(150.0, 250.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn remove_light(mut commands: Commands, directional_light: Query<Entity, With<DirectionalLight>>) {
    commands.remove_resource::<AmbientLight>();
    commands.entity(directional_light.single()).despawn();
}
