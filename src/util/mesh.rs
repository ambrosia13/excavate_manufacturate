use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub struct ChunkMeshBuilder {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl ChunkMeshBuilder {
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
        #[allow(clippy::needless_range_loop)]
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

    pub fn as_mesh(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);

        mesh.set_indices(Some(Indices::U32(self.indices)));

        mesh
    }
}

pub mod faces {
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

pub mod uvs {
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

pub mod normals {
    pub const NORMAL_Z_FRONT: [[f32; 3]; 4] = [[0.0, 0.0, 1.0]; 4];
    pub const NORMAL_Z_BACK: [[f32; 3]; 4] = [[0.0, 0.0, -1.0]; 4];
    pub const NORMAL_Y_FRONT: [[f32; 3]; 4] = [[0.0, 1.0, 0.0]; 4];
    pub const NORMAL_Y_BACK: [[f32; 3]; 4] = [[0.0, -1.0, 0.0]; 4];
    pub const NORMAL_X_FRONT: [[f32; 3]; 4] = [[1.0, 0.0, 0.0]; 4];
    pub const NORMAL_X_BACK: [[f32; 3]; 4] = [[-1.0, 0.0, 0.0]; 4];
}

#[allow(clippy::type_complexity)]
pub const NEIGHBOR_DATA: [(
    (i32, i32, i32), // Offset
    [[f32; 3]; 4],   // Vertices for the face
    [[f32; 3]; 4],   // Normals
    [[f32; 2]; 4],   // UV
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
