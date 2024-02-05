use bevy::gizmos::prelude::*;
use bevy::prelude::*;

use super::Player;

pub fn draw_crosshair(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform, &Transform), With<Player>>,
    mut gizmos: Gizmos,
) {
}
