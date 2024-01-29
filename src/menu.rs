use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::{state::GameState, world::render_distance::RenderDistance};

pub struct ExcavateManufacturateMenuPlugin;

impl Plugin for ExcavateManufacturateMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, menu_system.run_if(in_state(GameState::Menu)));
    }
}

fn menu_system(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut render_distance: ResMut<RenderDistance>,
) {
    egui::Window::new("Main Menu").show(contexts.ctx_mut(), |ui| {
        if ui.button("Start").clicked() {
            next_state.set(GameState::InGame);
        }

        let mut render_distance_chunks = render_distance.chunks();
        ui.add(egui::Slider::new(&mut render_distance_chunks, 2..=16).text("Render distance"));

        render_distance.set_to(render_distance_chunks as usize);

        if ui.button("Exit").clicked() {
            app_exit_events.send(AppExit);
        }
    });
}
