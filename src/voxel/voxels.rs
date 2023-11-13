use super::{Voxel, VoxelShape};

pub struct GrassBlock;

impl Voxel for GrassBlock {
    fn is_opaque(&self) -> bool {
        true
    }

    fn get_voxel_shape(&self) -> VoxelShape {
        VoxelShape::Default
    }
}

#[derive(Clone, Copy)]
pub struct Air;

impl Voxel for Air {
    fn is_opaque(&self) -> bool {
        false
    }

    fn get_voxel_shape(&self) -> VoxelShape {
        VoxelShape::Empty
    }
}
