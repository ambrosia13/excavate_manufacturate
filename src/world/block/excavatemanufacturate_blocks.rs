pub mod block_ids {
    use crate::world::block::BlockId;

    pub const GRASS: BlockId = BlockId("excavatemanufacturate/grass");
    pub const DIRT: BlockId = BlockId("excavatemanufacturate/dirt");
    pub const BEDROCK: BlockId = BlockId("excavatemanufacturate/bedrock");
    pub const STONE: BlockId = BlockId("excavatemanufacturate/stone");
}

pub mod block_types {
    use crate::world::block::Block;

    use super::block_ids;

    pub const GRASS: Block = Block::new_static(block_ids::GRASS);
    pub const DIRT: Block = Block::new_static(block_ids::DIRT);
    pub const BEDROCK: Block = Block::new_static(block_ids::BEDROCK);
    pub const STONE: Block = Block::new_static(block_ids::STONE);
}

pub mod block_data {
    use crate::world::block::static_block_data::{
        AtlasCoordinates, BlockHardnessLevel, BlockTextures, StaticBlockData, ToolType,
    };

    pub const GRASS: StaticBlockData = StaticBlockData {
        textures: BlockTextures {
            top: AtlasCoordinates {
                min: (0, 0),
                max: (15, 15),
            },
            sides: Some(AtlasCoordinates {
                min: (0, 16),
                max: (15, 31),
            }),
            bottom: Some(AtlasCoordinates {
                min: (0, 32),
                max: (15, 47),
            }),
        },
        hardness: BlockHardnessLevel::Hand,
    };
    pub const DIRT: StaticBlockData = StaticBlockData {
        textures: BlockTextures::from_single(AtlasCoordinates {
            min: (0, 32),
            max: (15, 47),
        }),
        hardness: BlockHardnessLevel::Hand,
    };
    pub const BEDROCK: StaticBlockData = StaticBlockData {
        textures: BlockTextures::from_single(AtlasCoordinates {
            min: (16, 16),
            max: (31, 31),
        }),
        hardness: BlockHardnessLevel::Unbreakable,
    };
    pub const STONE: StaticBlockData = StaticBlockData {
        textures: BlockTextures::from_single(AtlasCoordinates {
            min: (16, 0),
            max: (31, 15),
        }),
        hardness: BlockHardnessLevel::Tool(ToolType::Pickaxe, 0),
    };
}
