use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use super::Player;

#[derive(Resource)]
pub struct PlayerKeybinds {
    pub forward: KeyCode,
    pub back: KeyCode,

    pub left: KeyCode,
    pub right: KeyCode,

    pub up: KeyCode,
    pub down: KeyCode,

    pub break_block: MouseButton,
    pub place_block: MouseButton,
}

impl Default for PlayerKeybinds {
    fn default() -> Self {
        use KeyCode::*;

        Self {
            forward: W,
            back: S,
            left: A,
            right: D,
            up: Space,
            down: ShiftLeft,
            break_block: MouseButton::Left,
            place_block: MouseButton::Right,
        }
    }
}

pub fn setup_player_keybinds(mut commands: Commands) {
    commands.init_resource::<PlayerKeybinds>();
}

pub fn handle_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    player_keybinds: Res<PlayerKeybinds>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = player_query.single_mut();
    let mut translation = transform.translation;

    let movement_speed = 5.0 * time.delta_seconds();

    if input.pressed(player_keybinds.up) {
        translation += Vec3::Y * movement_speed;
    }
    if input.pressed(player_keybinds.down) {
        translation -= Vec3::Y * movement_speed;
    }
    if input.pressed(player_keybinds.left) {
        translation += transform.left() * movement_speed;
    }
    if input.pressed(player_keybinds.right) {
        translation += transform.right() * movement_speed;
    }

    let forward = transform.forward();
    let forward = Vec3::new(forward.x, 0.0, forward.z).normalize();

    if input.pressed(player_keybinds.forward) {
        translation += forward * movement_speed;
    }
    if input.pressed(player_keybinds.back) {
        translation -= forward * movement_speed;
    }

    transform.translation = translation;
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
