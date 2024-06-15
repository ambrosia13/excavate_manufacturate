use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PlayState {
    #[default]
    Playing,
    Paused,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameModeState {
    Survival,
    #[default]
    Creative,
}

pub fn set_state_to<T: States + Copy>(state: T) -> impl FnMut(ResMut<NextState<T>>) {
    move |mut next_state: ResMut<NextState<T>>| {
        info!("Setting the state to {:?}", state);
        next_state.set(state);
    }
}
