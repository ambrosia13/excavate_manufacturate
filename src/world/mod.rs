use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    tasks::AsyncComputeTaskPool,
};

use crate::state::MenuState;

use self::{
    collider::{ChunkColliderDisableEvent, ChunkColliderEnableEvent},
    world_access::{BlockDestroyEvent, BlockPlaceEvent},
};

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

pub const NUM_CHUNKS_RENDERED_PER_FRAME: usize = 1;

pub struct ExcavateManufacturateWorldPlugin;

impl Plugin for ExcavateManufacturateWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChunkColliderEnableEvent>()
            .add_event::<ChunkColliderDisableEvent>()
            .add_event::<BlockPlaceEvent>()
            .add_event::<BlockDestroyEvent>()
            .add_systems(Startup, (render_distance::setup, block::registry::setup))
            .add_systems(
                OnEnter(MenuState::InGame),
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
                        (
                            // Multithreaded chunk generation
                            generation::poll_generated_chunks,
                            generation::generate_chunks_multithreaded::<AsyncComputeTaskPool>,
                        ),
                        // generation::generate_chunks,
                        render::populate_chunk_spawn_queue,
                        (
                            (
                                render::spawn_chunks::<NUM_CHUNKS_RENDERED_PER_FRAME>,
                                render::despawn_chunks,
                            ),
                            (
                                collider::send_enable_chunk_colliders_near_mobs,
                                collider::send_disable_chunk_colliders_on_deserted_chunks,
                                collider::enable_chunk_colliders,
                                collider::disable_chunk_colliders,
                            ),
                        )
                            .chain(),
                    ),
                    (
                        world_access::apply_block_place_events,
                        world_access::apply_block_destroy_events,
                    ),
                )
                    .chain()
                    .run_if(in_state(MenuState::InGame)),
            )
            .add_systems(
                OnExit(MenuState::InGame),
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
        brightness: light_consts::lux::OVERCAST_DAY,
    });

    commands.insert_resource(DirectionalLightShadowMap { size: 1024 });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: false,
            ..Default::default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.01,
            maximum_distance: 4.0 * CHUNK_SIZE as f32,
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
