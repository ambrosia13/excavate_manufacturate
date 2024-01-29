use bevy::prelude::*;

use crate::{player, state::GameState, world};

pub struct ExcavateManufacturateGamePlugin;

impl Plugin for ExcavateManufacturateGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_flycam::NoCameraPlayerPlugin)
            .add_plugins((
                player::ExavateManufacturatePlayerPlugin,
                world::ExcavateManufacturateWorldPlugin,
            ))
            .add_systems(Update, exit_to_menu.run_if(in_state(GameState::InGame)));
    }
}

pub fn exit_to_menu(mut next_state: ResMut<NextState<GameState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Back) {
        next_state.set(GameState::Menu);
    }
}
