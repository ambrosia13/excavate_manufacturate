use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::{
    state,
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

use super::physics::{MobPhysicsBundle, MobVelocity};

pub mod camera;
pub mod interact;

pub struct ExcavateManufacturatePlayerPlugin;

impl Plugin for ExcavateManufacturatePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(state::MenuState::InGame), (setup, interact::setup))
            .add_systems(
                OnExit(state::MenuState::InGame),
                (cleanup, interact::cleanup),
            )
            .add_systems(
                OnEnter(state::GameModeState::Survival),
                enable_player_physics.run_if(in_state(state::MenuState::InGame)),
            )
            .add_systems(
                OnEnter(state::GameModeState::Creative),
                disable_player_physics.run_if(in_state(state::MenuState::InGame)),
            )
            .add_systems(
                Update,
                (
                    camera::update_player_chunk_and_block_pos,
                    camera::camera_rotation,
                    (
                        camera::creative_movement.run_if(in_state(state::GameModeState::Creative)),
                        camera::survival_movement.run_if(in_state(state::GameModeState::Survival)),
                        camera::copy_player_physics_transform_to_player_camera,
                    )
                        .chain()
                        .after(super::physics::resolve_mob_velocity),
                    (
                        interact::raycast,
                        (interact::draw_crosshair, interact::handle_destroy_block),
                    )
                        .chain(),
                )
                    .run_if(
                        in_state(state::MenuState::InGame)
                            .and_then(in_state(state::PlayState::Playing)),
                    ),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerPhysics;

pub fn setup(mut commands: Commands) {
    let x = rand::thread_rng().gen_range(-2000..=2000);
    let z = rand::thread_rng().gen_range(-2000..=2000);

    let player_block_pos = BlockPos::new(x, 50, z);
    let player_chunk_pos = ChunkPos::from(player_block_pos);

    let player_transform = Transform::from_translation(player_block_pos.as_vec3());

    // Player entity
    commands.spawn((
        Player,
        Camera3dBundle {
            transform: player_transform,
            ..Default::default()
        },
        player_block_pos,
        player_chunk_pos,
    ));

    // Player physics entity
    let mut physics = commands.spawn((
        PlayerPhysics,
        MobPhysicsBundle {
            transform_bundle: TransformBundle::from_transform(player_transform),
            collider: Collider::cuboid(0.3, 0.9, 0.3),
            ..Default::default()
        },
    ));

    physics.remove::<MobVelocity>();
}

pub fn cleanup(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    physics_query: Query<Entity, With<PlayerPhysics>>,
) {
    let player = player_query.single();
    let physics = physics_query.single();

    commands.entity(player).despawn();
    commands.entity(physics).despawn();
}

pub fn enable_player_physics(
    mut commands: Commands,
    physics_query: Query<Entity, With<PlayerPhysics>>,
) {
    let entity = physics_query.single();
    commands.entity(entity).insert(MobVelocity::default());
}

pub fn disable_player_physics(
    mut commands: Commands,
    physics_query: Query<Entity, With<PlayerPhysics>>,
) {
    let entity = physics_query.single();
    commands.entity(entity).remove::<MobVelocity>();
}
