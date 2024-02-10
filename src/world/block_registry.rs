use std::{
    io::Read,
    path::{Path, PathBuf},
};

use bevy::{prelude::*, utils::HashMap};
use image::DynamicImage;

use super::block;

#[derive(Resource)]
pub struct BlockRegistry {
    pub static_block_data: HashMap<&'static str, StaticBlockData>,
    pub atlas_size: (usize, usize),
}

// fn read_block_texture_file(namespace: &Path, file_name: &str) -> DynamicImage {
//     let error_message = format!("Couldn't open image at path {}", file_name);
//     image::open(namespace.join(file_name)).expect(&error_message)
// }

#[derive(Resource, Deref, DerefMut)]
pub struct TextureAtlasHandle(Handle<Image>);
