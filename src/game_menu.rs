use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};

use crate::{
    player::Player,
    state::{GameState, PlayerGameMode},
    util::block_pos::BlockPos,
    world::render_distance::RenderDistance,
};

pub struct ExcavateManufacturateGameMenuPlugin;

impl Plugin for ExcavateManufacturateGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, game_menu_system.run_if(in_state(GameState::InGame)));
    }
}

fn game_menu_system(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_game_mode: ResMut<NextState<PlayerGameMode>>,
    mut render_distance: ResMut<RenderDistance>,
    diagnostics: Res<DiagnosticsStore>,
    player_transform: Query<(&Transform, &BlockPos), With<Player>>,
) {
    egui::Window::new("Game Menu").show(contexts.ctx_mut(), |ui| {
        let (Transform { translation, .. }, block_pos) = player_transform.single();
        ui.label(format!(
            "Player position: {:?}; ({:.2}, {:.2}, {:.2})",
            block_pos, translation.x, translation.y, translation.z,
        ));

        if let Some(fps) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.value())
        {
            ui.label(format!("Fps: {:.3}", fps));
        }

        let mut render_distance_chunks = render_distance.chunks();
        ui.add(egui::Slider::new(&mut render_distance_chunks, 2..=16).text("Render distance"));

        render_distance.set_to(render_distance_chunks as usize);

        if ui.button("Creative mode").clicked() {
            next_game_mode.set(PlayerGameMode::Creative);
        }
        if ui.button("Survival mode").clicked() {
            next_game_mode.set(PlayerGameMode::Survival);
        }

        if ui.button("Exit to main menu").clicked() {
            next_state.set(GameState::Menu);
        }
    });
}
