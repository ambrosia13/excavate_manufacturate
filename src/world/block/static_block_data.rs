#[derive(Clone, Copy)]
pub struct AtlasCoordinates {
    pub min: (u16, u16),
    pub max: (u16, u16),
}

pub enum BlockHardnessLevel {
    Hand,
    ToolStrength(u8),
    Unbreakable,
}

pub struct StaticBlockData {
    pub atlas_coordinates: AtlasCoordinates,
    pub hardness: BlockHardnessLevel,
}
