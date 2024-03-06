use std::sync::Arc;

use bevy::prelude::*;

use crate::{util::block_pos::BlockPos, world::block};

use super::block::{registry::BlockRegistry, BlockData};

pub trait WorldGenerator {
    fn terrain_noise(&self, block_pos: BlockPos, registry: &BlockRegistry) -> BlockData;
}

pub struct OverworldGenerator;

impl WorldGenerator for OverworldGenerator {
    fn terrain_noise(&self, block_pos: BlockPos, registry: &BlockRegistry) -> BlockData {
        use block::excavatemanufacturate_blocks::block_names::*;

        match block_pos.y.cmp(&0) {
            std::cmp::Ordering::Less => BlockData::none(),
            std::cmp::Ordering::Equal => BlockData::some(registry.create_block(&GRASS).unwrap()),
            std::cmp::Ordering::Greater => {
                use noisy_bevy::*;

                let position = block_pos.as_vec3();
                let domain_warp = Vec3::ZERO;

                let hills_multiplier =
                    simplex_noise_2d(position.xz() * 0.005 + 10000.0) * 0.5 + 0.5;

                let hills_generator = |position: Vec3| {
                    position.y
                        + hills_multiplier
                            * 20.0
                            * simplex_noise_2d((position + domain_warp).xz() * 0.025)
                };

                let noise = hills_generator(position);

                let base_ground_level = 30.0;

                if noise < base_ground_level - 10.0 {
                    BlockData::some(registry.create_block(&STONE).unwrap())
                } else if noise < base_ground_level - 1.0 {
                    BlockData::some(registry.create_block(&DIRT).unwrap())
                } else if noise < base_ground_level {
                    BlockData::some(registry.create_block(&GRASS).unwrap())
                } else {
                    BlockData::none()
                }
            }
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct ActiveWorldGenerator<T: WorldGenerator>(Arc<T>);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ActiveWorldGenerator(Arc::new(OverworldGenerator)));
    info!("Set up world generator");
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<ActiveWorldGenerator<OverworldGenerator>>();
    info!("Cleaned up world generator");
}
