use std::sync::Arc;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

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

    pub fn generate_landscape_features(
        &self,
        block_pos: BlockPos,
        world: &mut ExcavateManufacturateWorld,
    ) {
        (self.landscape_feature_generator)(block_pos, world);
    }
}

pub fn setup(mut commands: Commands) {
    use block::excavatemanufacturate_blocks::block_types::*;

    let world_generator = WorldGenerator {
        terrain_noise: |block_pos| match block_pos.y.cmp(&0) {
            std::cmp::Ordering::Less => BlockData::none(),
            std::cmp::Ordering::Equal => BlockData::some(BEDROCK),
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
                    BlockData::some(STONE)
                } else if noise < base_ground_level - 1.0 {
                    BlockData::some(DIRT)
                } else if noise < base_ground_level {
                    BlockData::some(GRASS)
                } else {
                    BlockData::none()
                }
            }
        },
        landscape_feature_generator: |block_pos, world| {
            let boulder_generator =
                |block_pos: BlockPos, world: &mut ExcavateManufacturateWorld| {
                    let boulder_radius = 3;

                    for x in -boulder_radius..=boulder_radius {
                        for y in -boulder_radius..=boulder_radius {
                            for z in -boulder_radius..=boulder_radius {
                                let pos = block_pos + BlockPos::new(x, y, z);
                                let distance = block_pos.distance_squared(*pos);

                                if distance < 4 {
                                    world.set_block(pos, BlockData::some(STONE));
                                }
                            }
                        }
                    }
                };

            let rand = thread_rng().gen_range(0..1000);

            match rand {
                0..=9 => boulder_generator(block_pos, world),
                _ => {}
            }
        },
    };

    commands.insert_resource(WorldGeneratorResource(Arc::new(world_generator)));
    info!("Set up world generator");
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<WorldGeneratorResource>();
    info!("Cleaned up world generator");
}
