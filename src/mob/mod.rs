use bevy::prelude::*;

use crate::state;

pub mod physics;
pub mod player;

pub struct ExcavateManufacturateMobPlugin;

impl Plugin for ExcavateManufacturateMobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(state::MenuState::InGame), physics::setup)
            .add_systems(OnExit(state::MenuState::InGame), physics::cleanup)
            .add_systems(
                Update,
                (physics::tick_mob_gravity, physics::resolve_mob_velocity).run_if(
                    in_state(state::MenuState::InGame)
                        .and_then(in_state(state::PlayState::Playing)),
                ),
            );
    }
}
