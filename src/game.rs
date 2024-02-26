use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;

use crate::{
    player,
    state::{GameState, PlayingGameState},
    world,
};

pub struct ExcavateManufacturateGamePlugin;

impl Plugin for ExcavateManufacturateGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins((
                player::ExavateManufacturatePlayerPlugin,
                world::ExcavateManufacturateWorldPlugin,
            ))
            .add_systems(OnEnter(PlayingGameState::Playing), toggle_cursor_enabled)
            .add_systems(OnEnter(PlayingGameState::Paused), toggle_cursor_enabled)
            .add_systems(
                Update,
                (
                    exit_to_menu,
                    pause_game.run_if(in_state(PlayingGameState::Playing)),
                    unpause_game.run_if(in_state(PlayingGameState::Paused)),
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

pub fn toggle_cursor_enabled(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();

    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

pub fn pause_game(
    mut next_state: ResMut<NextState<PlayingGameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(PlayingGameState::Paused);
    }
}

pub fn unpause_game(
    mut next_state: ResMut<NextState<PlayingGameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(PlayingGameState::Playing);
    }
}

pub fn exit_to_menu(
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Backspace) {
        next_state.set(GameState::Menu);
    }
}
