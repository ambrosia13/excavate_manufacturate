use bevy::prelude::*;

pub enum BlockData {
    Empty,
    Full(BlockType),
}

pub enum BlockType {
    Debug,
}

impl BlockData {
    pub fn is_opaque(&self) -> bool {
        match self {
            BlockData::Empty => false,
            BlockData::Full(_) => true,
        }
    }

    pub fn has_geometry(&self) -> bool {
        match self {
            BlockData::Empty => false,
            BlockData::Full(_) => true,
        }
    }
}
