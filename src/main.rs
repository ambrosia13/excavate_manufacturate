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
mod menu;
mod player;
mod state;
mod util;
mod world;

fn main() {
    // let world_generator = |block_pos: BlockPos| {
    //     if block_pos.x % 2 == 0 {
    //         BlockData::Full(BlockType::Debug)
    //     } else {
    //         BlockData::Empty
    //     }
    // };

    // let chunk_1 = ChunkData::with_data(world_generator);
    // let chunk_2 = ChunkData::with_data(world_generator);

    // let mut world = ExcavateManufacturateWorld::new();

    // world.insert_chunk(ChunkPos::new(0, 0, 0), chunk_1);
    // world.insert_chunk(ChunkPos::new(1, 0, 0), chunk_2);

    // println!("{:?}", world.get_block(BlockPos::new(31, 1, 2)));

    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::Immediate,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(Msaa::Off)
        .add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
        ))
        .add_plugins((
            menu::ExcavateManufacturateMenuPlugin,
            game::ExcavateManufacturateGamePlugin,
        ))
        .run();
}
