use crate::util::mesh::BlockFace;

pub mod coord;

#[derive(Clone, Copy)]
pub struct AtlasCoordinates {
    pub min: (u16, u16),
    pub max: (u16, u16),
}

/// Contains coordinate data for each block. If only the top texture is specified, that texture is used for
/// all faces of the block.
pub struct BlockTextures {
    pub top: AtlasCoordinates,

    /// Defaults to top face coordinates if not set
    pub sides: Option<AtlasCoordinates>,

    /// Defaults to top face coordinates if not set
    pub bottom: Option<AtlasCoordinates>,
}

impl BlockTextures {
    /// Creates a [`BlockTextures`] from one set of atlas coordinates.
    pub const fn from_single(atlas_coordinates: AtlasCoordinates) -> Self {
        Self {
            top: atlas_coordinates,
            sides: None,
            bottom: None,
        }
    }

    pub fn get_coords(&self, face: BlockFace) -> AtlasCoordinates {
        match face {
            BlockFace::Top => self.top,
            BlockFace::Side => self.sides.unwrap_or(self.top),
            BlockFace::Bottom => self.bottom.unwrap_or(self.top),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockHardnessLevel {
    Hand,
    ToolStrength(u8),
    Unbreakable,
}

pub struct StaticBlockData {
    pub textures: BlockTextures,
    pub hardness: BlockHardnessLevel,
}
