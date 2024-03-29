use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts};

use crate::{state::MenuState, world::render_distance::RenderDistance};

pub struct ExcavateManufacturateMainMenuPlugin;

impl Plugin for ExcavateManufacturateMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            main_menu_system.run_if(in_state(MenuState::MainMenu)),
        );
    }
}

fn main_menu_system(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<MenuState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut render_distance: ResMut<RenderDistance>,
    input: Res<ButtonInput<KeyCode>>,
) {
    egui::Window::new("Main Menu").show(contexts.ctx_mut(), |ui| {
        if ui.button("Start").clicked() || input.just_pressed(KeyCode::Space) {
            next_state.set(MenuState::InGame);
        }

        let mut render_distance_chunks = render_distance.chunks();
        ui.add(egui::Slider::new(&mut render_distance_chunks, 2..=16).text("Render distance"));

        render_distance.set_to(render_distance_chunks as usize);

        if ui.button("Exit").clicked() || input.just_pressed(KeyCode::Backspace) {
            app_exit_events.send(AppExit);
        }
    });
}
