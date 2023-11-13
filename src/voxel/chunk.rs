use super::{gen::WorldGenerator, ChunkPos, Voxel, WorldPos, CHUNK_SIZE, PADDED_CHUNK_SIZE};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use std::any::Any;

pub struct VoxelChunkComponent(VoxelChunk);

pub struct VoxelChunk {
    offset: ChunkPos,
    voxels: [Box<dyn Voxel>; PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE],
    mesh: Option<Mesh>,
    empty: bool,
}

impl VoxelChunk {
    fn linearize_chunk_pos(pos: (usize, usize, usize)) -> usize {
        pos.0 + pos.1 * CHUNK_SIZE + pos.2 * CHUNK_SIZE * CHUNK_SIZE
    }

    pub fn new<T: WorldGenerator>(offset: ChunkPos, generator: &mut T) -> Self {
        let mut voxels = std::array::from_fn::<
            Box<dyn Voxel>,
            { PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE * PADDED_CHUNK_SIZE },
            _,
        >(|_| Box::new(super::voxels::Air));

        let mut empty = true;

        for x in 0..PADDED_CHUNK_SIZE {
            for y in 0..PADDED_CHUNK_SIZE {
                for z in 0..PADDED_CHUNK_SIZE {
                    let current_pos = super::chunk_pos_to_world_pos(offset)
                        + WorldPos::new(x as i32, y as i32, z as i32)
                        - IVec3::ONE;

                    let current_voxel: Box<dyn Voxel> = generator.generate_terrain(current_pos);

                    // if any voxel in the chunk is not empty, the chunk can't be considered empty
                    match current_voxel.get_voxel_shape() {
                        super::VoxelShape::Empty => {}
                        _ => empty = false,
                    }

                    voxels[Self::linearize_chunk_pos((x, y, z))] = current_voxel;
                }
            }
        }

        Self {
            offset,
            voxels,
            mesh: None,
            empty,
        }
    }

    fn get_mesh(&self) -> Mesh {
        let mut builder = MeshBuilder::new();

        const NEIGHBOR_DATA: [(
            (isize, isize, isize), // Offset
            [[f32; 3]; 4],         // Vertices for the face
            [[f32; 3]; 4],         // Normals
            [[f32; 2]; 4],         // UV
        ); 6] = [
            // Positive z
            (
                (0, 0, 1),
                faces::FACE_Z_FRONT,
                normals::NORMAL_Z_FRONT,
                uvs::UV_Z_FRONT,
            ),
            // Negative z
            (
                (0, 0, -1),
                faces::FACE_Z_BACK,
                normals::NORMAL_Z_BACK,
                uvs::UV_Z_BACK,
            ),
            // Positive y
            (
                (0, 1, 0),
                faces::FACE_Y_FRONT,
                normals::NORMAL_Y_FRONT,
                uvs::UV_Y_FRONT,
            ),
            // Negative y
            (
                (0, -1, 0),
                faces::FACE_Y_BACK,
                normals::NORMAL_Y_BACK,
                uvs::UV_Y_BACK,
            ),
            // Positive x
            (
                (1, 0, 0),
                faces::FACE_X_FRONT,
                normals::NORMAL_X_FRONT,
                uvs::UV_X_FRONT,
            ),
            // Negative x
            (
                (-1, 0, 0),
                faces::FACE_X_BACK,
                normals::NORMAL_X_BACK,
                uvs::UV_X_BACK,
            ),
        ];

        for x in 1..(PADDED_CHUNK_SIZE - 1) as isize {
            for y in 1..(PADDED_CHUNK_SIZE - 1) as isize {
                for z in 1..(PADDED_CHUNK_SIZE - 1) as isize {
                    // Skip this mesh logic for empty voxels
                    if self
                        .get_at((x as usize, y as usize, z as usize))
                        .has_no_geometry()
                    {
                        continue;
                    }

                    // Iterate over each neighbor; if there is a neighbor voxel, add geometry information in that direction
                    for ((dx, dy, dz), face, normals, uvs) in NEIGHBOR_DATA.iter() {
                        let neighbor =
                            self.get_at(((x + dx) as usize, (y + dy) as usize, (z + dz) as usize));

                        match neighbor.get_voxel_shape() {
                            super::VoxelShape::Empty => builder.add_face(
                                *face,
                                *normals,
                                *uvs,
                                Vec3::new(x as f32, y as f32, z as f32),
                            ),
                            _ => {}
                        }
                    }
                }
            }
        }

        builder.to_mesh()
    }

    pub fn offset(&self) -> ChunkPos {
        self.offset
    }

    pub fn get_at(&self, index: (usize, usize, usize)) -> &Box<dyn Voxel> {
        &self.voxels[Self::linearize_chunk_pos(index)]
    }

    pub fn get_at_mut(&mut self, index: (usize, usize, usize)) -> &mut Box<dyn Voxel> {
        &mut self.voxels[Self::linearize_chunk_pos(index)]
    }

    pub fn try_get_at(&self, index: (isize, isize, isize)) -> Option<&Box<dyn Voxel>> {
        let index = (index.0 as usize, index.1 as usize, index.2 as usize);

        if (0..CHUNK_SIZE).contains(&index.0)
            && (0..CHUNK_SIZE).contains(&index.1)
            && (0..CHUNK_SIZE).contains(&index.2)
        {
            Some(&self.voxels[Self::linearize_chunk_pos(index)])
        } else {
            None
        }
    }

    pub fn try_get_at_mut(&mut self, index: (isize, isize, isize)) -> Option<&mut Box<dyn Voxel>> {
        let index = (index.0 as usize, index.1 as usize, index.2 as usize);

        if (0..CHUNK_SIZE).contains(&index.0)
            && (0..CHUNK_SIZE).contains(&index.1)
            && (0..CHUNK_SIZE).contains(&index.2)
        {
            Some(&mut self.voxels[Self::linearize_chunk_pos(index)])
        } else {
            None
        }
    }

    pub fn get_or_create_mesh(&mut self) -> Mesh {
        if let Some(mesh) = &self.mesh {
            mesh.clone()
        } else {
            let mesh = self.get_mesh();
            self.mesh = Some(mesh);

            self.mesh.as_ref().unwrap().clone()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn set_at(&mut self, index: (usize, usize, usize), new_voxel: Box<dyn Voxel>) {
        let old_voxel = &mut self.voxels[Self::linearize_chunk_pos(index)];
        old_voxel.on_destroy();

        self.voxels[Self::linearize_chunk_pos(index)] = new_voxel;
    }

    pub fn destroy_at(&mut self, index: (usize, usize, usize)) {
        self.set_at(index, Box::new(super::voxels::Air));
    }

    pub fn tick(&mut self) {
        for voxel in self.voxels.iter_mut() {
            voxel.tick();
        }
    }
}

struct MeshBuilder {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

mod faces {
    pub const FACE_Z_FRONT: [[f32; 3]; 4] = [
        [0.0, 0.0, 1.0], // Bottom left
        [0.0, 1.0, 1.0], // Top left
        [1.0, 0.0, 1.0], // Bottom right
        [1.0, 1.0, 1.0], // Top right
    ];
    pub const FACE_Z_BACK: [[f32; 3]; 4] = [
        [1.0, 0.0, 0.0], // Bottom right
        [1.0, 1.0, 0.0], // Top right
        [0.0, 0.0, 0.0], // Bottom left
        [0.0, 1.0, 0.0], // Top left
    ];
    pub const FACE_Y_FRONT: [[f32; 3]; 4] = [
        [0.0, 1.0, 1.0], // Front left
        [0.0, 1.0, 0.0], // Back left
        [1.0, 1.0, 1.0], // Front right
        [1.0, 1.0, 0.0], // Back right
    ];
    pub const FACE_Y_BACK: [[f32; 3]; 4] = [
        [0.0, 0.0, 0.0], // Front left
        [0.0, 0.0, 1.0], // Back left
        [1.0, 0.0, 0.0], // Front right
        [1.0, 0.0, 1.0], // Back right
    ];
    pub const FACE_X_FRONT: [[f32; 3]; 4] = [
        [1.0, 0.0, 1.0], // Front bottom
        [1.0, 1.0, 1.0], // Front top
        [1.0, 0.0, 0.0], // Back bottom
        [1.0, 1.0, 0.0], // Back top
    ];
    pub const FACE_X_BACK: [[f32; 3]; 4] = [
        [0.0, 0.0, 0.0], // Front bottom
        [0.0, 1.0, 0.0], // Front top
        [0.0, 0.0, 1.0], // Back bottom
        [0.0, 1.0, 1.0], // Back top
    ];
}

mod uvs {
    pub const UV_Z_FRONT: [[f32; 2]; 4] = [
        [0.0, 0.0], // Bottom left
        [0.0, 1.0], // Top left
        [1.0, 0.0], // Bottom right
        [1.0, 1.0], // Top right
    ];
    pub const UV_Z_BACK: [[f32; 2]; 4] = [
        [1.0, 0.0], // Bottom right
        [1.0, 1.0], // Top right
        [0.0, 0.0], // Bottom left
        [0.0, 1.0], // Top left
    ];
    pub const UV_Y_FRONT: [[f32; 2]; 4] = [
        [0.0, 1.0], // Front left
        [0.0, 0.0], // Back left
        [1.0, 1.0], // Front right
        [1.0, 0.0], // Back right
    ];
    pub const UV_Y_BACK: [[f32; 2]; 4] = [
        [0.0, 0.0], // Front left
        [0.0, 1.0], // Back left
        [1.0, 0.0], // Front right
        [1.0, 1.0], // Back right
    ];
    pub const UV_X_FRONT: [[f32; 2]; 4] = [
        [1.0, 0.0], // Front bottom
        [1.0, 1.0], // Front top
        [0.0, 0.0], // Back bottom
        [0.0, 1.0], // Back top
    ];
    pub const UV_X_BACK: [[f32; 2]; 4] = [
        [0.0, 0.0], // Front bottom
        [0.0, 1.0], // Front top
        [1.0, 0.0], // Back bottom
        [1.0, 1.0], // Back top
    ];
}

mod normals {
    pub const NORMAL_Z_FRONT: [[f32; 3]; 4] = [[0.0, 0.0, 1.0]; 4];
    pub const NORMAL_Z_BACK: [[f32; 3]; 4] = [[0.0, 0.0, -1.0]; 4];
    pub const NORMAL_Y_FRONT: [[f32; 3]; 4] = [[0.0, 1.0, 0.0]; 4];
    pub const NORMAL_Y_BACK: [[f32; 3]; 4] = [[0.0, -1.0, 0.0]; 4];
    pub const NORMAL_X_FRONT: [[f32; 3]; 4] = [[1.0, 0.0, 0.0]; 4];
    pub const NORMAL_X_BACK: [[f32; 3]; 4] = [[-1.0, 0.0, 0.0]; 4];
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn get_face_indices(starting_index: u32) -> [u32; 6] {
        [
            starting_index,
            starting_index + 2,
            starting_index + 1,
            starting_index + 2,
            starting_index + 3,
            starting_index + 1,
        ]
    }

    pub fn add_face(
        &mut self,
        mut face: [[f32; 3]; 4],
        normals: [[f32; 3]; 4],
        uvs: [[f32; 2]; 4],
        offset: Vec3,
    ) {
        for i in 0..4 {
            for j in 0..3 {
                face[i][j] += offset[j];
            }
        }

        let starting_index = self.vertices.len();

        self.vertices.extend_from_slice(&face);
        self.normals.extend_from_slice(&normals);
        self.uvs.extend_from_slice(&uvs);

        self.indices
            .extend_from_slice(&Self::get_face_indices(starting_index as u32));
    }

    pub fn to_mesh(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);

        mesh.set_indices(Some(Indices::U32(self.indices)));

        mesh
    }
}
