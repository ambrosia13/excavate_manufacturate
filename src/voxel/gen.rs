use bevy::prelude::*;

use super::{Voxel, WorldPos};

pub trait WorldGenerator {
    fn generate_terrain(&mut self, pos: WorldPos) -> Box<dyn Voxel>;
}

pub struct DefaultWorldGenerator;

impl WorldGenerator for DefaultWorldGenerator {
    fn generate_terrain(&mut self, pos: WorldPos) -> Box<dyn Voxel> {
        let height: i32 = ((pos.y as f32 * 0.3).sin() * 4.0) as i32;

        if pos.y % 2 == 0 {
            Box::new(super::voxels::GrassBlock)
        } else {
            Box::new(super::voxels::Air)
        }
    }
}
