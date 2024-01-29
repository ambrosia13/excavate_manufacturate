use bevy::prelude::*;

use crate::state::GameState;

pub struct ExavateManufacturatePlayerPlugin;

impl Plugin for ExavateManufacturatePlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_player)
            .add_systems(OnExit(GameState::InGame), despawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 30.0, 0.0),
            ..Default::default()
        },
        bevy_flycam::FlyCam,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    info!("Initialized player camera");
}

fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let player = player_query.single();
    commands.entity(player).despawn();

    info!("Removed player camera");
}
