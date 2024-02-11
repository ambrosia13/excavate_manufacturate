use bevy::{prelude::*, utils::HashMap};

use super::{excavatemanufacturate_blocks, static_block_data::StaticBlockData, BlockId};

#[derive(Resource)]
pub struct BlockRegistry {
    pub static_block_data: HashMap<BlockId, StaticBlockData>,
    pub atlas_size: (usize, usize),
}

impl BlockRegistry {
    pub fn get_block_data(&self, id: BlockId) -> &StaticBlockData {
        self.static_block_data.get(&id).unwrap()
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct TextureAtlasHandle(Handle<Image>);

pub fn setup_block_registry(mut commands: Commands, mut assets: ResMut<Assets<Image>>) {
    let mut assets_directory = bevy::asset::io::file::FileAssetReader::get_base_path();
    assets_directory.push("assets");
    assets_directory.push("excavatemanufacturate");
    assets_directory.push("textures");

    let atlas_dynamic_image = image::open(assets_directory.join("atlas.png")).unwrap();

    let atlas_size = (
        atlas_dynamic_image.width() as usize,
        atlas_dynamic_image.height() as usize,
    );

    let atlas_image = Image::from_dynamic(atlas_dynamic_image, true);
    commands.insert_resource(TextureAtlasHandle(assets.add(atlas_image)));

    let mut block_registry = BlockRegistry {
        static_block_data: HashMap::new(),
        atlas_size,
    };

    block_registry.static_block_data.insert(
        excavatemanufacturate_blocks::block_ids::GRASS,
        excavatemanufacturate_blocks::block_data::GRASS,
    );
    block_registry.static_block_data.insert(
        excavatemanufacturate_blocks::block_ids::DIRT,
        excavatemanufacturate_blocks::block_data::DIRT,
    );
    block_registry.static_block_data.insert(
        excavatemanufacturate_blocks::block_ids::BEDROCK,
        excavatemanufacturate_blocks::block_data::BEDROCK,
    );

    commands.insert_resource(block_registry);
}
