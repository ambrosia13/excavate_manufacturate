use bevy::prelude::*;

use crate::util::block_pos::BlockPos;

use super::Block;

#[derive(Component)]
pub struct BlockEntity;

#[derive(Bundle)]
pub struct DynamicBlockDataBundle {
    block_pos: BlockPos,
}

#[derive(Component, Deref, DerefMut)]
pub struct Storage<T, const SLOTS: usize>([T; SLOTS]);

#[derive(Component, Deref, DerefMut)]
pub struct Age(std::time::Duration);

impl Age {
    pub fn new() -> Self {
        Self(std::time::Duration::ZERO)
    }
}

pub fn update_age(mut query: Query<&mut Age>, time: Res<Time>) {
    for mut age in query.iter_mut() {
        **age += time.elapsed();
    }
}
