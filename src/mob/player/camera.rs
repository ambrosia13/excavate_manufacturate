use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::{
    keybinds::Keybinds,
    mob::physics::MobVelocity,
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
};

use super::{Player, PlayerPhysics};

pub fn update_player_chunk_and_block_pos(
    mut query: Query<(&mut ChunkPos, &mut BlockPos, &Transform), With<Player>>,
) {
    let (mut chunk_pos, mut block_pos, transform) = query.single_mut();

    *block_pos = BlockPos::from(transform.translation);
    *chunk_pos = ChunkPos::from(*block_pos);
}

pub fn creative_movement(
    player_query: Query<&Transform, With<Player>>,
    mut physics_query: Query<&mut KinematicCharacterController, With<PlayerPhysics>>,
    keybinds: Res<Keybinds>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let transform = player_query.single();
    let mut controller = physics_query.single_mut();

    let forward = transform.forward();
    let forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();

    let movement_speed = 12.5 * time.delta_seconds();
    let mut player_movement = Vec3::ZERO;

    if input.pressed(keybinds.up) {
        player_movement += Vec3::Y;
    }
    if input.pressed(keybinds.down) {
        player_movement -= Vec3::Y;
    }

    if input.pressed(keybinds.left) {
        player_movement += Vec3::from(transform.left());
    }
    if input.pressed(keybinds.right) {
        player_movement += Vec3::from(transform.right());
    }
    if input.pressed(keybinds.forward) {
        player_movement += forward;
    }
    if input.pressed(keybinds.back) {
        player_movement -= forward;
    }

    // If the player movement isn't zero, that means we moved
    if let Some(player_movement) = player_movement.try_normalize() {
        controller.translation = if let Some(translation) = controller.translation {
            Some(player_movement * movement_speed + translation)
        } else {
            Some(player_movement * movement_speed)
        };
    }
}

pub fn survival_movement(
    player_query: Query<&Transform, With<Player>>,
    mut physics_query: Query<
        (&mut MobVelocity, &KinematicCharacterControllerOutput),
        With<PlayerPhysics>,
    >,
    time: Res<Time>,
    keybinds: Res<Keybinds>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut velocity, output)) = physics_query.get_single_mut() else {
        return;
    };

    let transform = player_query.single();

    let forward = transform.forward();
    let forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();

    let movement_velocity = 0.5 * time.delta_seconds();
    let jump_velocity = 0.0325;

    let mut player_movement = Vec3::ZERO;
    let mut player_jump = Vec3::ZERO;

    if input.pressed(keybinds.up) && output.grounded {
        player_jump += Vec3::Y * jump_velocity;
    }

    if input.pressed(keybinds.left) {
        player_movement += Vec3::from(transform.left());
    }
    if input.pressed(keybinds.right) {
        player_movement += Vec3::from(transform.right());
    }
    if input.pressed(keybinds.forward) {
        player_movement += forward;
    }
    if input.pressed(keybinds.back) {
        player_movement -= forward;
    }

    let player_movement = player_movement.normalize_or_zero() * movement_velocity;

    velocity.0 += Vec3::new(player_movement.x, player_jump.y, player_movement.z);

    let friction_coefficient = 15.0;
    let air_resistance_coefficient = 10.0;

    if output.grounded {
        let decay = (-friction_coefficient * time.delta_seconds()).exp();

        velocity.x *= decay;
        velocity.z *= decay;
    } else {
        let decay = (-air_resistance_coefficient * time.delta_seconds()).exp();

        velocity.x *= decay;
        velocity.z *= decay;
    }
}

pub fn camera_rotation(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut motion_event_reader: EventReader<MouseMotion>,
) {
    let mut transform = player_query.single_mut();
    let window = window_query.single();

    let sensitivity = 1e-4;

    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    for ev in motion_event_reader.read() {
        let window_scale = window.height().min(window.width());

        pitch -= (sensitivity * ev.delta.y * window_scale).to_radians();
        yaw -= (sensitivity * ev.delta.x * window_scale).to_radians();
    }

    pitch = pitch.clamp((-89.5f32).to_radians(), (89.5f32).to_radians());

    transform.rotation =
        Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
}

pub fn copy_player_physics_transform_to_player_camera(
    mut set: ParamSet<(
        // Player entity
        Query<&mut Transform, With<Player>>,
        // Player physics entity
        Query<&Transform, With<PlayerPhysics>>,
    )>,
) {
    let physics_translation = {
        let physics_query = set.p1();
        let single = physics_query.get_single();

        match single {
            Ok(transform) => transform.translation,

            // If this is an error, this means the player doesn't currently have physics attached to it
            Err(_) => return,
        }
    };

    let mut player_query = set.p0();
    let mut player_transform = player_query.single_mut();

    player_transform.translation = physics_translation + Vec3::new(0.0, 0.8, 0.0);
}
