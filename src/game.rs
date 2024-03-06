use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;

use crate::{mob, state, world};

pub struct ExcavateManufacturateGamePlugin;

impl Plugin for ExcavateManufacturateGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins((
                mob::ExcavateManufacturateMobPlugin,
                mob::player::ExcavateManufacturatePlayerPlugin,
                world::ExcavateManufacturateWorldPlugin,
            ))
            .add_systems(OnEnter(state::MenuState::MainMenu), enable_cursor)
            .add_systems(
                OnEnter(state::PlayState::Playing),
                disable_cursor.run_if(in_state(state::MenuState::InGame)),
            )
            .add_systems(
                OnEnter(state::PlayState::Paused),
                enable_cursor.run_if(in_state(state::MenuState::InGame)),
            )
            .add_systems(
                Update,
                (
                    exit_to_menu,
                    pause_game.run_if(in_state(state::PlayState::Playing)),
                    unpause_game.run_if(in_state(state::PlayState::Paused)),
                )
                    .run_if(in_state(state::MenuState::InGame)),
            );
    }
}

pub fn enable_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();

    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
}

pub fn disable_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();

    window.cursor.grab_mode = CursorGrabMode::Confined;
    window.cursor.visible = false;
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
    mut next_state: ResMut<NextState<state::PlayState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(state::PlayState::Paused);
    }
}

pub fn unpause_game(
    mut next_state: ResMut<NextState<state::PlayState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(state::PlayState::Playing);
    }
}

pub fn exit_to_menu(
    mut next_state: ResMut<NextState<state::MenuState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Backspace) {
        next_state.set(state::MenuState::MainMenu);
    }
}
