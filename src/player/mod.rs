use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

use crate::{
    state::{GameState, PlayerGameMode, PlayingGameState},
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

use self::movement::PlayerTransformCopyEvent;

pub mod cursor;
pub mod interact;
pub mod keybinds;
pub mod movement;
pub mod physics;

pub struct ExavateManufacturatePlayerPlugin;

impl Plugin for ExavateManufacturatePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerTransformCopyEvent>()
            .add_systems(Startup, keybinds::setup_player_keybinds)
            .add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(OnExit(GameState::InGame), cleanup)
            .add_systems(
                Update,
                (
                    update_player_pos,
                    cursor::draw_crosshair,
                    (
                        // interact::destroy_block,
                        interact::raycast
                            .pipe(interact::draw_crosshair)
                            .pipe(interact::handle_destroy_block)
                            .pipe(interact::finish_interaction),
                        // Player movement
                        (
                            (
                                movement::handle_player_rotation,
                                // Creative movement, just flight without physics
                                movement::handle_player_flight
                                    .run_if(in_state(PlayerGameMode::Creative)),
                                // Survival movement, includes physics & gravity
                                (
                                    movement::apply_mob_gravity,
                                    movement::apply_mob_velocity,
                                    movement::handle_player_movement,
                                )
                                    .chain()
                                    .run_if(in_state(PlayerGameMode::Survival)),
                            ),
                            movement::copy_mob_physics,
                        )
                            .chain(),
                    )
                        .run_if(in_state(PlayingGameState::Playing)),
                )
                    .run_if(in_state(GameState::InGame)),
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
        MobVelocity(Vec3::ZERO),
    ));

    info!("Set up player");
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
