use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_rapier3d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};

use super::{keybinds::PlayerKeybinds, Player, PlayerGravity, PlayerPhysics};

pub fn handle_player_movement(
    player_query: Query<&Transform, With<Player>>,
    mut player_physics_query: Query<&mut KinematicCharacterController, With<PlayerPhysics>>,
    player_keybinds: Res<PlayerKeybinds>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let transform = player_query.single();
    let mut controller = player_physics_query.single_mut();

    let forward = transform.forward();
    let forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();

    let movement_speed = 7.5 * time.delta_seconds();
    let mut player_movement = Vec3::ZERO;

    if input.pressed(player_keybinds.up) {
        player_movement += Vec3::Y;
    }
    if input.pressed(player_keybinds.down) {
        player_movement -= Vec3::Y;
    }
    if input.pressed(player_keybinds.left) {
        player_movement += transform.left();
    }
    if input.pressed(player_keybinds.right) {
        player_movement += transform.right();
    }
    if input.pressed(player_keybinds.forward) {
        player_movement += forward;
    }
    if input.pressed(player_keybinds.back) {
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

pub fn handle_player_rotation(
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

    pitch = pitch.clamp((-170.0f32).to_radians(), (170.0f32).to_radians());

    transform.rotation =
        Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
}

pub fn update_player_gravity(
    mut physics_query: Query<
        (&mut PlayerGravity, &KinematicCharacterControllerOutput),
        With<PlayerPhysics>,
    >,
    time: Res<Time>,
) {
    let (mut gravity, output) = physics_query.single_mut();

    let velocity_change = -1.0;

    if output.grounded {
        *gravity = PlayerGravity(0.0);
    } else {
        gravity.0 += velocity_change * time.delta_seconds();
    }
}

pub fn apply_player_gravity(
    mut physics_query: Query<
        (&PlayerGravity, &mut KinematicCharacterController),
        With<PlayerPhysics>,
    >,
) {
    let (gravity, mut controller) = physics_query.single_mut();

    controller.translation = if let Some(translation) = controller.translation {
        Some(gravity.0 * Vec3::Y + translation)
    } else {
        Some(gravity.0 * Vec3::Y)
    };
}

#[derive(Event)]
pub struct PlayerTransformCopyEvent(Vec3);

pub fn send_physics_translation(
    physics_query: Query<&Transform, With<PlayerPhysics>>,
    mut events: EventWriter<PlayerTransformCopyEvent>,
) {
    let translation = physics_query.single().translation;
    events.send(PlayerTransformCopyEvent(translation));
}

pub fn recv_physics_translation_into_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut events: EventReader<PlayerTransformCopyEvent>,
) {
    let mut player_transform = player_query.single_mut();

    for PlayerTransformCopyEvent(translation) in events.read() {
        player_transform.translation = *translation;
    }
}
