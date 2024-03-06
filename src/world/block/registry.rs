use std::sync::Arc;

use bevy::{prelude::*, render::render_asset::RenderAssetUsages, utils::HashMap};

use super::{
    excavatemanufacturate_blocks, static_block_data::StaticBlockData, Block, BlockId, BlockName,
};

#[derive(Resource, Deref)]
pub struct BlockRegistryResource(Arc<BlockRegistry>);

pub struct BlockRegistry {
    pub block_id_map: HashMap<BlockName, BlockId>,
    pub static_block_data: HashMap<BlockId, StaticBlockData>,
    pub atlas_size: (usize, usize),
}

impl BlockRegistry {
    pub fn create_block(&self, name: &BlockName) -> Option<Block> {
        Block::from_name(name, self)
    }

    pub fn get_block_id(&self, name: &BlockName) -> Option<BlockId> {
        self.block_id_map.get(name).cloned()
    }

    pub fn get_block_data(&self, id: BlockId) -> &StaticBlockData {
        self.static_block_data
            .get(&id)
            .unwrap_or_else(|| panic!("Block id {:?} doesn't exist in the block registry", id))
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct TextureAtlasHandle(Handle<Image>);

pub fn setup(mut commands: Commands, mut assets: ResMut<Assets<Image>>) {
    let mut assets_directory = bevy::asset::io::file::FileAssetReader::get_base_path();
    assets_directory.push("assets");
    assets_directory.push("excavatemanufacturate");
    assets_directory.push("textures");

    let atlas_dynamic_image =
        image::open(assets_directory.join("atlas.png")).expect("Couldn't load texture atlas image");

    let atlas_size = (
        atlas_dynamic_image.width() as usize,
        atlas_dynamic_image.height() as usize,
    );

    let atlas_image =
        Image::from_dynamic(atlas_dynamic_image, true, RenderAssetUsages::RENDER_WORLD);
    commands.insert_resource(TextureAtlasHandle(assets.add(atlas_image)));

    let mut block_registry = BlockRegistry {
        block_id_map: HashMap::new(),
        static_block_data: HashMap::new(),
        atlas_size,
    };

    use excavatemanufacturate_blocks::*;

    let block_names = {
        [
            block_names::GRASS,
            block_names::DIRT,
            block_names::BEDROCK,
            block_names::STONE,
        ]
    };

    for (next_block_id, name) in block_names.into_iter().enumerate() {
        block_registry
            .block_id_map
            .insert(name, BlockId(next_block_id as u32));
    }

    block_registry.static_block_data.insert(
        block_registry.get_block_id(&block_names::GRASS).unwrap(),
        excavatemanufacturate_blocks::block_data::GRASS,
    );
    block_registry.static_block_data.insert(
        block_registry.get_block_id(&block_names::DIRT).unwrap(),
        excavatemanufacturate_blocks::block_data::DIRT,
    );
    block_registry.static_block_data.insert(
        block_registry.get_block_id(&block_names::BEDROCK).unwrap(),
        excavatemanufacturate_blocks::block_data::BEDROCK,
    );
    block_registry.static_block_data.insert(
        block_registry.get_block_id(&block_names::STONE).unwrap(),
        excavatemanufacturate_blocks::block_data::STONE,
    );

    commands.insert_resource(BlockRegistryResource(Arc::new(block_registry)));
}
