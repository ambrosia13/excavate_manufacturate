use bevy::prelude::*;
use state::GameState;

mod game;
mod game_menu;
mod main_menu;
mod player;
mod state;
mod util;
mod world;

fn main() {
    App::new()
        .add_state::<GameState>()
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
        .run();
}
