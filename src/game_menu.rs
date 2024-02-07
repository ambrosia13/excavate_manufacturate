use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    player::Player, state::GameState, util::block_pos::BlockPos,
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
    mut render_distance: ResMut<RenderDistance>,
    player_transform: Query<&Transform, With<Player>>,
) {
    egui::Window::new("Game Menu").show(contexts.ctx_mut(), |ui| {
        ui.label(format!(
            "Player position: {:?}, {}",
            BlockPos::from(player_transform.single().translation),
            player_transform.single().translation,
        ));

        let mut render_distance_chunks = render_distance.chunks();
        ui.add(egui::Slider::new(&mut render_distance_chunks, 2..=16).text("Render distance"));

        render_distance.set_to(render_distance_chunks as usize);

        if ui.button("Exit to main menu").clicked() {
            next_state.set(GameState::Menu);
        }
    });
}
