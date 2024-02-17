use std::sync::Arc;

use bevy::prelude::*;

use crate::{util::block_pos::BlockPos, world::block};

use super::{block::BlockData, world_access::ExcavateManufacturateWorld};

#[derive(Resource, Deref, DerefMut)]
pub struct WorldGeneratorResource(Arc<WorldGenerator>);

pub struct WorldGenerator {
    pub terrain_noise: fn(BlockPos) -> BlockData,
    pub landscape_feature_generator: fn(BlockPos, &mut ExcavateManufacturateWorld),
}

impl WorldGenerator {
    pub fn generate_terrain_noise(&self, block_pos: BlockPos) -> BlockData {
        (self.terrain_noise)(block_pos)
    }
}

pub fn setup(mut commands: Commands) {
    let world_generator = WorldGenerator {
        terrain_noise: |block_pos| {
            use block::excavatemanufacturate_blocks::block_types::*;

            match block_pos.y.cmp(&0) {
                std::cmp::Ordering::Less => BlockData::None,
                std::cmp::Ordering::Equal => BlockData::Some(BEDROCK),
                std::cmp::Ordering::Greater => {
                    let position = block_pos.as_vec3();

                    let hills_multiplier =
                        noisy_bevy::simplex_noise_2d(position.xz() * 0.005 + 10000.0) * 0.5 + 0.5;

                    let hills_generator = |position: Vec3| {
                        position.y
                            + hills_multiplier
                                * 20.0
                                * noisy_bevy::simplex_noise_2d(position.xz() * 0.025)
                    };

                    let noise = hills_generator(position);

                    let base_ground_level = 30.0;

                    if noise < base_ground_level - 10.0 {
                        BlockData::Some(STONE)
                    } else if noise < base_ground_level - 1.0 {
                        BlockData::Some(DIRT)
                    } else if noise < base_ground_level {
                        BlockData::Some(GRASS)
                    } else {
                        BlockData::None
                    }
                }
            }
        },
        landscape_feature_generator: |_, _| {},
    };

    commands.insert_resource(WorldGeneratorResource(Arc::new(world_generator)));
    info!("Set up world generator");
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<WorldGeneratorResource>();
    info!("Cleaned up world generator");
}
