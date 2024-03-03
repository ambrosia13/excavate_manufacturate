use bevy::prelude::*;
use state::{GameModeState, MenuState, PlayState};

mod game;
mod game_menu;
mod keybinds;
mod main_menu;
mod mob;
mod player;
mod state;
mod util;
mod world;

fn main() {
    App::new()
        .init_state::<MenuState>()
        .init_state::<PlayState>()
        .init_state::<GameModeState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::Immediate,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(bevy_egui::EguiPlugin)
        .insert_resource(Msaa::Off)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins((
            main_menu::ExcavateManufacturateMainMenuPlugin,
            game::ExcavateManufacturateGamePlugin,
            game_menu::ExcavateManufacturateGameMenuPlugin,
        ))
        .add_systems(Startup, keybinds::setup)
        .run();
}
