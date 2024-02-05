use bevy::{prelude::*, utils::hashbrown::HashMap};
use state::GameState;
use world::{
    block::{BlockData, BlockType},
    chunk::ChunkData,
};

use crate::{
    util::{block_pos::BlockPos, chunk_pos::ChunkPos},
    world::world_access::ExcavateManufacturateWorld,
};

mod game;
mod game_menu;
mod main_menu;
mod player;
mod state;
mod util;
mod world;

fn main() {
    // let world_generator = |block_pos: BlockPos| {
    //     if block_pos == BlockPos::new(21, 0, 1) {
    //         BlockData::Full(BlockType::Debug)
    //     } else {
    //         BlockData::Empty
    //     }
    // };

    // let chunk_1 = ChunkData::with_data(|block_pos| {
    //     world_generator(block_pos + BlockPos::from(ChunkPos::new(0, 0, 0)))
    // });
    // let chunk_2 = ChunkData::with_data(|block_pos| {
    //     world_generator(block_pos + BlockPos::from(ChunkPos::new(1, 0, 0)))
    // });

    // let mut world = ExcavateManufacturateWorld::new();

    // world.insert_chunk(ChunkPos::new(0, 0, 0), chunk_1);
    // world.insert_chunk(ChunkPos::new(1, 0, 0), chunk_2);

    // let chunk = world.get_chunk(ChunkPos::new(1, 0, 0)).unwrap();

    // let block_pos = BlockPos::new(21, 0, 1);
    // println!(
    //     "world: {:?} vs chunk: {:?}",
    //     world.get_block(block_pos),
    //     chunk.get(block_pos)
    // );

    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::Immediate,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(bevy_egui::EguiPlugin)
        .insert_resource(Msaa::Off)
        .add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
        ))
        .add_plugins((
            main_menu::ExcavateManufacturateMainMenuPlugin,
            game::ExcavateManufacturateGamePlugin,
            game_menu::ExcavateManufacturateGameMenuPlugin,
        ))
        .run();
}
