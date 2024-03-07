use std::sync::Arc;

use bevy::{prelude::*, render::render_asset::RenderAssetUsages, utils::HashMap};

use super::{
    excavatemanufacturate_blocks, static_block_data::StaticBlockData, Block, BlockId, BlockName,
};

#[derive(Resource, Deref)]
pub struct BlockRegistryResource(Arc<BlockRegistry>);

pub struct BlockRegistry {
    pub block_ids: HashMap<BlockName, BlockId>,
    pub static_block_data: HashMap<BlockId, StaticBlockData>,
    pub atlas_size: (usize, usize),
}

impl BlockRegistry {
    fn create(atlas_size: (usize, usize)) -> Self {
        use excavatemanufacturate_blocks::*;

        let mut block_ids = HashMap::new();
        let mut static_block_data = HashMap::new();

        let block_names = [
            block_names::GRASS,
            block_names::DIRT,
            block_names::BEDROCK,
            block_names::STONE,
        ];

        for (next_block_id, name) in block_names.into_iter().enumerate() {
            block_ids.insert(name, BlockId(next_block_id as u16));
        }

        static_block_data.insert(
            *block_ids.get(&block_names::GRASS).unwrap(),
            excavatemanufacturate_blocks::block_data::GRASS,
        );
        static_block_data.insert(
            *block_ids.get(&block_names::DIRT).unwrap(),
            excavatemanufacturate_blocks::block_data::DIRT,
        );
        static_block_data.insert(
            *block_ids.get(&block_names::BEDROCK).unwrap(),
            excavatemanufacturate_blocks::block_data::BEDROCK,
        );
        static_block_data.insert(
            *block_ids.get(&block_names::STONE).unwrap(),
            excavatemanufacturate_blocks::block_data::STONE,
        );

        Self {
            block_ids,
            static_block_data,
            atlas_size,
        }
    }

    pub fn create_block(&self, name: &BlockName) -> Option<Block> {
        Block::from_name(name, self)
    }

    pub fn get_block_id(&self, name: &BlockName) -> Option<BlockId> {
        self.block_ids.get(name).cloned()
    }

    pub fn get_block_data(&self, id: BlockId) -> &StaticBlockData {
        // It's ok to panic here because BlockId is never manually created; it should always be valid.
        self.static_block_data
            .get(&id)
            .unwrap_or_else(|| panic!("Block id {:?} doesn't exist in the block registry", id))
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct TextureAtlasHandle(Handle<Image>);

#[derive(Resource, Deref, DerefMut)]
pub struct Mods(Vec<ModInfo>);

pub struct ModInfo {
    pub namespace: &'static str,
    pub blocks: Vec<&'static str>,
}

pub fn setup(mut commands: Commands, mut assets: ResMut<Assets<Image>>) {
    let mut assets_directory = bevy::asset::io::file::FileAssetReader::get_base_path();
    assets_directory.push("assets");
    assets_directory.push("excavatemanufacturate");
    assets_directory.push("textures");

    // let mut block_textures = Vec::new();

    // for mod_info in mods.iter() {
    //     let mut directory = assets_directory.join(mod_info.namespace);
    //     directory.push("textures");
    //     directory.push("block");

    //     for &block in mod_info.blocks.iter() {
    //         let texture_file = directory.join(format!("{}.png", block));

    //         let Ok(image) = image::open(texture_file) else {
    //             panic!(
    //                 "Failed to read image when parsing textures for mod {}: {}",
    //                 mod_info.namespace, block
    //             );
    //         };

    //         block_textures.push(image);
    //     }
    // }

    // let namespaces = ["excavatemanufacturate"];

    // let excavatemanufacturate = ModInfo {
    //     namespace: "excavatemanufacturate",
    //     blocks: vec!["grass, dirt, bedrock, stone"],
    // };

    // for namespace in namespaces {
    //     let mut directory = assets_directory.join(namespace);
    //     directory.push("textures");
    //     directory.push("block");
    // }

    let atlas_dynamic_image =
        image::open(assets_directory.join("atlas.png")).expect("Couldn't load texture atlas image");

    let atlas_size = (
        atlas_dynamic_image.width() as usize,
        atlas_dynamic_image.height() as usize,
    );

    let atlas_image =
        Image::from_dynamic(atlas_dynamic_image, true, RenderAssetUsages::RENDER_WORLD);
    commands.insert_resource(TextureAtlasHandle(assets.add(atlas_image)));

    let block_registry = BlockRegistry::create(atlas_size);
    commands.insert_resource(BlockRegistryResource(Arc::new(block_registry)));
}
