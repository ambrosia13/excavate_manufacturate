use bevy::{prelude::*, utils::HashMap};
use bevy_flycam::prelude::*;
use voxel::{chunk::VoxelChunk, world::VoxelWorld, ChunkPos, Voxel};

mod voxel;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, create_world)
        .run();
}

fn create_world(mut commands: Commands) {
    let world = VoxelWorld::new();
    commands.insert_resource(world);
}

fn generate_chunks_in_view_distance(mut commands: Commands, mut world: ResMut<VoxelWorld>) {}

fn spawn_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut chunk = VoxelChunk::new(IVec3::ZERO, &mut voxel::gen::DefaultWorldGenerator);

    commands.spawn(PbrBundle {
        mesh: meshes.add(chunk.get_or_create_mesh()),
        material: materials.add(StandardMaterial {
            base_color: Color::ALICE_BLUE,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(50.0, 200.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}
