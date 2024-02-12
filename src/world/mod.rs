use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};

use crate::state::GameState;

pub mod block;
pub mod chunk;
pub mod generation;
pub mod render;
pub mod render_distance;
pub mod world_access;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_SIZE_INT: i32 = CHUNK_SIZE as i32;

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
                world_access::setup_world_access,
                generation::setup_world_generator,
                generation::setup_chunk_generation_structures,
                render::setup_chunk_spawning_structures,
            ),
        )
        .add_systems(
            Update,
            (
                (
                    // Multithreaded chunk generation
                    generation::poll_generated_chunks,
                    generation::generate_chunks_on_thread_pool,
                ),
                render::populate_chunk_spawn_queue,
                (
                    // Multithreaded chunk meshing
                    render::spawn_chunks,
                    render::poll_spawned_chunks,
                ),
                render::despawn_chunks,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            OnExit(GameState::InGame),
            (
                remove_light,
                world_access::remove_world_access,
                generation::remove_world_generator,
                generation::remove_chunk_generation_structures,
                render::remove_chunk_spawning_structures,
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
