use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    state::GameState,
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

use self::movement::PlayerTransformCopyEvent;

pub mod cursor;
pub mod interact;
pub mod keybinds;
pub mod movement;

pub struct ExavateManufacturatePlayerPlugin;

impl Plugin for ExavateManufacturatePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerTransformCopyEvent>()
            .add_systems(Startup, keybinds::setup_player_keybinds)
            .add_systems(OnEnter(GameState::InGame), setup_player)
            .add_systems(OnExit(GameState::InGame), despawn_player)
            .add_systems(
                Update,
                (
                    update_player_pos,
                    interact::destroy_block,
                    cursor::draw_crosshair,
                    // Player movement
                    (
                        (
                            movement::handle_player_movement,
                            movement::handle_player_rotation,
                            movement::update_player_gravity,
                            movement::apply_player_gravity,
                        ),
                        (
                            movement::send_physics_translation,
                            movement::recv_physics_translation_into_player,
                        )
                            .chain(),
                    )
                        .chain(),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_plugins(RapierDebugRenderPlugin::default());
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerPhysics;

#[derive(Component)]
pub struct PlayerGravity(f32);

fn setup_player(mut commands: Commands) {
    let player_block_pos = BlockPos::new(0, 50, 0);
    let player_chunk_pos = ChunkPos::from(player_block_pos);

    let player_transform = Transform::from_translation(player_block_pos.as_vec3());

    commands.spawn((
        Player,
        Camera3dBundle {
            transform: player_transform,
            ..Default::default()
        },
        player_chunk_pos,
        player_block_pos,
    ));

    commands.spawn((
        PlayerPhysics,
        Collider::cuboid(0.4, 0.9, 0.4),
        RigidBody::KinematicVelocityBased,
        KinematicCharacterController::default(),
        TransformBundle {
            local: player_transform,
            ..Default::default()
        },
        PlayerGravity(0.0),
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
