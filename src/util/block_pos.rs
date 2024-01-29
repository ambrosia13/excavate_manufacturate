use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::prelude::*;

use crate::world::CHUNK_SIZE_INT;

use super::chunk_pos::ChunkPos;

#[derive(Component, Clone, Copy, Deref, DerefMut, PartialEq, Eq, Hash)]
pub struct BlockPos(IVec3);

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }

    pub fn as_chunk_offset(&self) -> BlockPos {
        BlockPos::new(
            self.x.rem_euclid(CHUNK_SIZE_INT),
            self.y.rem_euclid(CHUNK_SIZE_INT),
            self.z.rem_euclid(CHUNK_SIZE_INT),
        )
    }

    pub fn inner(&self) -> IVec3 {
        self.0
    }
}

impl From<IVec3> for BlockPos {
    fn from(value: IVec3) -> Self {
        Self(value)
    }
}

impl From<Vec3> for BlockPos {
    fn from(value: Vec3) -> Self {
        Self::from(value.as_ivec3())
    }
}

impl From<ChunkPos> for BlockPos {
    fn from(value: ChunkPos) -> Self {
        Self(value.inner() * CHUNK_SIZE_INT)
    }
}

impl Add for BlockPos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for BlockPos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for BlockPos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for BlockPos {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
