use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::{
    state::{GameModeState, MenuState, PlayState},
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

pub mod interact;
pub mod keybinds;
pub mod movement;
pub mod physics;

pub struct ExavateManufacturatePlayerPlugin;

impl Plugin for ExavateManufacturatePlayerPlugin {
    fn build(&self, app: &mut App) {
        let setup_systems = (setup, interact::setup);
        let cleanup_systems = (cleanup, interact::cleanup);

        let interaction_systems = (
            interact::raycast,
            (interact::draw_crosshair, interact::handle_destroy_block),
        )
            .chain();

        let player_movement_systems = (
            movement::handle_player_rotation,
            // Creative movement, just flight without physics
            movement::handle_player_flight.run_if(in_state(GameModeState::Creative)),
            // Survival movement, includes physics & gravity
            movement::handle_player_movement.run_if(in_state(GameModeState::Survival)),
        );

        let physics_systems = (
            (movement::apply_mob_gravity, movement::apply_mob_velocity),
            movement::copy_mob_physics,
        )
            .chain();

        app.add_systems(Startup, keybinds::setup)
            .add_systems(OnEnter(MenuState::InGame), setup_systems)
            .add_systems(OnExit(MenuState::InGame), cleanup_systems)
            .add_systems(OnEnter(GameModeState::Survival), add_mob_velocity_to_player)
            .add_systems(
                OnExit(GameModeState::Survival),
                remove_mob_velocity_from_player,
            )
            .add_systems(
                Update,
                (
                    update_player_pos,
                    (
                        interaction_systems,
                        player_movement_systems,
                        physics_systems,
                        interact::spawn_ball,
                    )
                        .run_if(in_state(PlayState::Playing)),
                )
                    .run_if(in_state(MenuState::InGame)),
            )
            .add_plugins(RapierDebugRenderPlugin::default().disabled());
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerPhysics;

#[derive(Component)]
pub struct Mob;

/// Should be placed on the physics entity as a reference to the mob entity.
#[derive(Component)]
pub struct ReferenceToMob(pub Entity);

#[derive(Component, Deref, DerefMut)]
pub struct MobVelocity(pub Vec3);

fn setup(mut commands: Commands) {
    let x = rand::thread_rng().gen_range(-2000..=2000);
    let z = rand::thread_rng().gen_range(-2000..=2000);

    let player_block_pos = BlockPos::new(x, 50, z);
    let player_chunk_pos = ChunkPos::from(player_block_pos);

    let player_transform = Transform::from_translation(player_block_pos.as_vec3());

    let player_entity = commands
        .spawn((
            Player,
            Mob,
            Camera3dBundle {
                transform: player_transform,
                ..Default::default()
            },
            player_chunk_pos,
            player_block_pos,
        ))
        .id();

    commands.spawn((
        PlayerPhysics,
        Collider::cuboid(0.3, 0.9, 0.3),
        RigidBody::KinematicVelocityBased,
        KinematicCharacterController::default(),
        TransformBundle {
            local: player_transform,
            ..Default::default()
        },
        // The player physics entity has a reference to the player entity
        ReferenceToMob(player_entity),
    ));

    info!("Set up player");
}

fn add_mob_velocity_to_player(
    mut commands: Commands,
    player_physics: Query<Entity, With<PlayerPhysics>>,
) {
    let entity = player_physics.single();
    commands.entity(entity).insert(MobVelocity(Vec3::ZERO));
}

fn remove_mob_velocity_from_player(
    mut commands: Commands,
    player_physics: Query<Entity, With<PlayerPhysics>>,
) {
    let entity = player_physics.single();
    commands.entity(entity).remove::<MobVelocity>();
}

fn update_player_pos(
    mut player_query: Query<(&mut ChunkPos, &mut BlockPos, &Transform), With<Player>>,
) {
    let (mut chunk_pos, mut block_pos, transform) = player_query.single_mut();
    *block_pos = BlockPos::from(transform.translation);
    *chunk_pos = ChunkPos::from(*block_pos);
}

fn cleanup(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    physics_query: Query<Entity, With<PlayerPhysics>>,
) {
    let player = player_query.single();
    commands.entity(player).despawn();

    let physics = physics_query.single();
    commands.entity(physics).despawn();

    info!("Cleaned up player");
}
