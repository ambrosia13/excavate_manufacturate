use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::prelude::*;

use crate::world::CHUNK_SIZE_INT;

use super::block_pos::BlockPos;

#[derive(Component, Clone, Copy, Deref, DerefMut, PartialEq, Eq, Hash)]
pub struct ChunkPos(IVec3);

impl ChunkPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }

    pub fn inner(&self) -> IVec3 {
        self.0
    }
}

impl From<IVec3> for ChunkPos {
    fn from(value: IVec3) -> Self {
        Self(value)
    }
}

impl From<BlockPos> for ChunkPos {
    fn from(value: BlockPos) -> Self {
        Self(value.inner() / CHUNK_SIZE_INT)
    }
}

impl Add for ChunkPos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for ChunkPos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for ChunkPos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for ChunkPos {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::fmt::Debug for ChunkPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ChunkPos({}, {}, {})", self.x, self.y, self.z))
    }
}

/// In relation to the player. (0, 0, 0) refers to the chunk that the player is currently in.
#[derive(Deref, DerefMut, Clone, Copy)]
pub struct LocalChunkPos(ChunkPos);

impl LocalChunkPos {
    pub fn from(chunk_pos_world: ChunkPos, player_chunk_pos: ChunkPos) -> Self {
        Self(chunk_pos_world - player_chunk_pos)
    }
}
