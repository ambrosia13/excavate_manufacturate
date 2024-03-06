use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    keybinds::Keybinds,
    util::{self, block_pos::BlockPos, raytrace::Hit},
    world::{
        block::{registry::BlockRegistryResource, static_block_data::BlockHardnessLevel},
        world_access::{BlockDestroyEvent, ExcavateManufacturateWorld},
    },
};

use super::Player;

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
    em_world: Res<ExcavateManufacturateWorld>,
    block_registry: Res<BlockRegistryResource>,

    input: Res<ButtonInput<MouseButton>>,
    keybinds: Res<Keybinds>,
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
