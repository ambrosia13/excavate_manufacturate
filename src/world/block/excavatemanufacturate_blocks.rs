pub mod block_ids {
    use crate::world::block::BlockId;

    pub const NAMESPACE: &str = "excavatemanufacturate";

    pub const GRASS: BlockId = 0;
    pub const DIRT: BlockId = 1;
    pub const BEDROCK: BlockId = 2;
}

pub mod block_types {
    use crate::world::block::BlockType;

    use super::block_ids;

    pub const GRASS: BlockType = BlockType::new_static(block_ids::GRASS);
    pub const DIRT: BlockType = BlockType::new_static(block_ids::DIRT);
    pub const BEDROCK: BlockType = BlockType::new_static(block_ids::BEDROCK);
}

pub mod block_data {
    use crate::world::block::static_block_data::{
        AtlasCoordinates, BlockHardnessLevel, StaticBlockData,
    };

    pub const GRASS: StaticBlockData = StaticBlockData {
        atlas_coordinates: AtlasCoordinates {
            min: (0, 0),
            max: (15, 15),
        },
        hardness: BlockHardnessLevel::Hand,
    };
    pub const DIRT: StaticBlockData = StaticBlockData {
        atlas_coordinates: AtlasCoordinates {
            min: (0, 16),
            max: (15, 31),
        },
        hardness: BlockHardnessLevel::Hand,
    };
    pub const BEDROCK: StaticBlockData = StaticBlockData {
        atlas_coordinates: AtlasCoordinates {
            min: (16, 16),
            max: (31, 31),
        },
        hardness: BlockHardnessLevel::Unbreakable,
    };
}
