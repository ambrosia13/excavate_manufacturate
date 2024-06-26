use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};

use crate::{
    mob::player::Player,
    state::{GameModeState, MenuState},
    util::block_pos::BlockPos,
    world::{
        generation::GeneratedChunkTask,
        render::{ChunkSpawnQueue, SpawnedChunks},
        render_distance::RenderDistance,
        world_access::ExcavateManufacturateWorld,
    },
};

pub struct ExcavateManufacturateGameMenuPlugin;

impl Plugin for ExcavateManufacturateGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (game_menu_system, performance_metrics_system).run_if(in_state(MenuState::InGame)),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn game_menu_system(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<MenuState>>,
    mut next_game_mode: ResMut<NextState<GameModeState>>,
    mut render_distance: ResMut<RenderDistance>,
    player_transform: Query<(&Transform, &BlockPos), With<Player>>,
) {
    egui::Window::new("Game Menu").show(contexts.ctx_mut(), |ui| {
        let (Transform { translation, .. }, block_pos) = player_transform.single();
        ui.label(format!(
            "Player position: {:?}; ({:.2}, {:.2}, {:.2})",
            block_pos, translation.x, translation.y, translation.z,
        ));

        let mut render_distance_chunks = render_distance.chunks();
        ui.add(egui::Slider::new(&mut render_distance_chunks, 2..=16).text("Render distance"));

        render_distance.set_to(render_distance_chunks as usize);

        if ui.button("Creative mode").clicked() {
            next_game_mode.set(GameModeState::Creative);
        }
        if ui.button("Survival mode").clicked() {
            next_game_mode.set(GameModeState::Survival);
        }

        if ui.button("Exit to main menu").clicked() {
            next_state.set(MenuState::MainMenu);
        }
    });
}

fn performance_metrics_system(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    spawned_chunks: Res<SpawnedChunks>,
    em_world: Res<ExcavateManufacturateWorld>,
    spawn_queue: Res<ChunkSpawnQueue>,
    entity_query: Query<()>,
    tasks_query: Query<(), With<GeneratedChunkTask>>,
) {
    egui::Window::new("Performance Metrics").show(contexts.ctx_mut(), |ui| {
        if let Some(fps) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            ui.label(format!("Fps: {:.3}", fps));
        }

        ui.label(format!("Entity count: {}", entity_query.iter().count()));
        ui.label(format!("Chunks stored: {}", em_world.total_chunk_count()));
        ui.label(format!("Chunk gen tasks: {}", tasks_query.iter().count()));
        ui.label(format!("Chunks rendered: {}", spawned_chunks.len()));
        ui.label(format!("Chunks queued to render: {}", spawn_queue.len()));
    });
}
