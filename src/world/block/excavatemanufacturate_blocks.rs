pub mod block_names {
    use crate::world::block::BlockName;

    pub const GRASS: BlockName = BlockName("excavatemanufacturate/grass");
    pub const DIRT: BlockName = BlockName("excavatemanufacturate/dirt");
    pub const BEDROCK: BlockName = BlockName("excavatemanufacturate/bedrock");
    pub const STONE: BlockName = BlockName("excavatemanufacturate/stone");
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
