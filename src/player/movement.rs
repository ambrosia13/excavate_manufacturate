use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_rapier3d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};

use super::{keybinds::PlayerKeybinds, MobVelocity, Player, PlayerPhysics, ReferenceToMob};

pub fn handle_player_flight(
    player_query: Query<&Transform, With<Player>>,
    mut player_physics_query: Query<&mut KinematicCharacterController, With<PlayerPhysics>>,
    player_keybinds: Res<PlayerKeybinds>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let transform = player_query.single();
    let mut controller = player_physics_query.single_mut();

    let forward = transform.forward();
    let forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();

    let movement_speed = 12.5 * time.delta_seconds();
    let mut player_movement = Vec3::ZERO;

    if input.pressed(player_keybinds.up) {
        player_movement += Vec3::Y;
    }
    if input.pressed(player_keybinds.down) {
        player_movement -= Vec3::Y;
    }

    if input.pressed(player_keybinds.left) {
        player_movement += Vec3::from(transform.left());
    }
    if input.pressed(player_keybinds.right) {
        player_movement += Vec3::from(transform.right());
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

pub fn handle_player_movement(
    player_query: Query<&Transform, With<Player>>,
    mut physics_query: Query<
        (
            &mut MobVelocity,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<PlayerPhysics>,
    >,
    time: Res<Time>,
    player_keybinds: Res<PlayerKeybinds>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let (mut velocity, Some(output)) = physics_query.single_mut() else {
        return;
    };

    let transform = player_query.single();

    let forward = transform.forward();
    let forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();

    let movement_velocity = 0.5 * time.delta_seconds();
    let jump_velocity = 0.0325;

    let mut player_movement = Vec3::ZERO;
    let mut player_jump = Vec3::ZERO;

    if input.pressed(player_keybinds.up) && output.grounded {
        player_jump += Vec3::Y * jump_velocity;
    }

    if input.pressed(player_keybinds.left) {
        player_movement += Vec3::from(transform.left());
    }
    if input.pressed(player_keybinds.right) {
        player_movement += Vec3::from(transform.right());
    }
    if input.pressed(player_keybinds.forward) {
        player_movement += forward;
    }
    if input.pressed(player_keybinds.back) {
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

    pitch = pitch.clamp((-89.5f32).to_radians(), (89.5f32).to_radians());

    transform.rotation =
        Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
}

pub fn apply_mob_gravity(
    mut physics_query: Query<(
        &mut MobVelocity,
        Option<&KinematicCharacterControllerOutput>,
    )>,
    time: Res<Time>,
) {
    let velocity_change = -0.1 * time.delta_seconds();

    for (mut velocity, output) in physics_query.iter_mut() {
        let Some(output) = output else {
            info!("Kinematic character controller output doesn't exist, skipping gravity update");
            continue;
        };

        if output.grounded {
            velocity.y = 0.0;
        } else {
            velocity.y += velocity_change;
        }
    }
}

pub fn apply_mob_velocity(
    mut physics_query: Query<(&MobVelocity, &mut KinematicCharacterController)>,
) {
    let terminal_velocity = 1.0;

    for (MobVelocity(velocity), mut controller) in physics_query.iter_mut() {
        let velocity = velocity.clamp_length_max(terminal_velocity);

        controller.translation = if let Some(translation) = controller.translation {
            Some(velocity + translation)
        } else {
            Some(velocity)
        };
    }
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
        player_transform.translation = *translation + Vec3::new(0.0, 0.8, 0.0);
    }
}

#[allow(clippy::type_complexity)]
/// Copies the physics transform's translation onto the corresponding mob transform's translation.
pub fn copy_mob_physics(
    mut set: ParamSet<(
        // Mob transform query
        Query<(&mut Transform, Has<Player>)>,
        // Mob physics query
        Query<(&Transform, &ReferenceToMob), With<KinematicCharacterController>>,
    )>,
) {
    let mut to_copy = Vec::new();

    for (transform, ReferenceToMob(entity)) in set.p1().iter() {
        to_copy.push((transform.translation, *entity));
    }

    let mut transform_query = set.p0();

    for (translation, entity) in to_copy {
        let Ok((mut transform, is_player)) = transform_query.get_mut(entity) else {
            warn!("Physics to mob reference is invalid!");
            return;
        };

        transform.translation = translation;

        if is_player {
            transform.translation += Vec3::new(0.0, 0.8, 0.0);
        }
    }
}
