use bevy::gizmos::prelude::*;
use bevy::prelude::*;

use super::Player;

pub fn draw_crosshair(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform, &Transform), With<Player>>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform, player_transform) = camera_query.single();

    let Some(ray) = camera.viewport_to_world(camera_transform, Vec2::new(0.0, 0.0)) else {
        return;
    };

    let crosshair_position = ray.get_point(0.01);

    gizmos.circle(crosshair_position, -ray.direction, 0.01, Color::WHITE);
}
