use bevy::prelude::*;
use bevy_rapier3d::{
    control::KinematicCharacterController, dynamics::RigidBody, geometry::Collider,
};

use crate::{
    util::{self, block_pos::BlockPos, raytrace::Hit},
    world::{
        block::{registry::BlockRegistry, static_block_data::BlockHardnessLevel, BlockData},
        render::ChunkSpawnQueue,
        world_access::{BlockDestroyEvent, ExcavateManufacturateWorld},
    },
};

use super::{keybinds::PlayerKeybinds, Mob, MobVelocity, Player, ReferenceToMob};

#[derive(Resource, Deref)]
pub struct PlayerRaycast(pub Option<Hit>);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(PlayerRaycast(None));
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<PlayerRaycast>();
}

pub fn raycast(
    player_transform: Query<&Transform, With<Player>>,
    em_world: Res<ExcavateManufacturateWorld>,
    mut player_raycast: ResMut<PlayerRaycast>,
) {
    let player_transform = player_transform.single();

    player_raycast.0 = util::raytrace::raytrace_dda(
        player_transform.translation,
        Vec3::from(player_transform.forward()),
        30,
        em_world.hit_evaluator(),
    )
}

pub fn draw_crosshair(player_raycast: Res<PlayerRaycast>, mut gizmos: Gizmos) {
    if let PlayerRaycast(Some(hit)) = *player_raycast {
        gizmos.sphere(hit.position, Quat::IDENTITY, 0.25, Color::WHITE);
    }
}

pub fn handle_destroy_block(
    player_raycast: Res<PlayerRaycast>,

    mut block_destroy_events: EventWriter<BlockDestroyEvent>,
    em_world: ResMut<ExcavateManufacturateWorld>,
    block_registry: Res<BlockRegistry>,

    input: Res<ButtonInput<MouseButton>>,
    keybinds: Res<PlayerKeybinds>,
) {
    if let PlayerRaycast(Some(hit)) = *player_raycast {
        if input.just_pressed(keybinds.break_block) {
            let block_pos = BlockPos::from(hit.position - 0.1 * hit.normal);
            let block_data = em_world.get_block(block_pos);

            let block_can_be_destroyed = block_data.is_some_and(|block_data| {
                block_data.as_ref().is_some_and(|block_type| {
                    block_registry.get_block_data(block_type.id).hardness
                        != BlockHardnessLevel::Unbreakable
                })
            });

            if block_can_be_destroyed {
                block_destroy_events.send(BlockDestroyEvent::create(block_pos, &em_world));
            }
        }
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    player_transform: Query<&Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if input.just_pressed(KeyCode::KeyB) {
        let transform = player_transform.single();
        let position = transform.translation + Vec3::from(transform.forward()) * 10.0;

        let transform = Transform::from_translation(position);

        // Mob entity
        let mob = commands
            .spawn(MaterialMeshBundle {
                mesh: meshes.add(Sphere::new(0.5).mesh()),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    ..Default::default()
                }),
                transform,
                ..Default::default()
            })
            .insert(Mob)
            .id();

        // Physics entity
        commands.spawn((
            RigidBody::KinematicVelocityBased,
            KinematicCharacterController::default(),
            TransformBundle {
                local: transform,
                ..Default::default()
            },
            MobVelocity(Vec3::ZERO),
            ReferenceToMob(mob),
            Collider::ball(0.5),
        ));
    }
}
