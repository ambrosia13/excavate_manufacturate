use bevy::prelude::*;

use crate::util::chunk_pos::LocalChunkPos;

use super::CHUNK_SIZE_INT;

#[derive(Resource)]
pub struct RenderDistance(usize);

impl RenderDistance {
    pub fn new(chunks: usize) -> Self {
        Self(chunks)
    }

    pub fn set_to(&mut self, new_render_distance: usize) {
        self.0 = new_render_distance;
    }

    pub fn chunks(&self) -> i32 {
        self.0 as i32
    }

    pub fn blocks(&self) -> i32 {
        self.0 as i32 * CHUNK_SIZE_INT
    }

    pub fn contains(&self, pos: LocalChunkPos) -> bool {
        pos.x.abs() <= self.chunks() && pos.y.abs() <= self.chunks() && pos.z.abs() <= self.chunks()
    }
}

const INITIAL_RENDER_DISTANCE: usize = 8;

pub fn setup_render_distance(mut commands: Commands) {
    commands.insert_resource(RenderDistance::new(INITIAL_RENDER_DISTANCE));

    info!(
        "Initialized render distance to {} chunks",
        INITIAL_RENDER_DISTANCE
    );
}
