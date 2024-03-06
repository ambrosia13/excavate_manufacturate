use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct GravityAcceleration(pub f32);

#[derive(Resource, Deref, DerefMut)]
pub struct TerminalVelocity(pub f32);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(GravityAcceleration(-0.1));
    commands.insert_resource(TerminalVelocity(1.0));
}

pub fn cleanup(mut commands: Commands) {
    commands.remove_resource::<GravityAcceleration>();
    commands.remove_resource::<TerminalVelocity>();
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct MobVelocity(pub Vec3);

#[derive(Bundle)]
pub struct MobPhysicsBundle {
    pub mob_velocity: MobVelocity,
    pub transform_bundle: TransformBundle,
    pub collider: Collider,
    pub character_controller: KinematicCharacterController,
    pub rigid_body: RigidBody,
}

impl Default for MobPhysicsBundle {
    fn default() -> Self {
        Self {
            mob_velocity: Default::default(),
            transform_bundle: Default::default(),
            collider: Default::default(),
            character_controller: Default::default(),
            rigid_body: RigidBody::KinematicVelocityBased,
        }
    }
}

pub fn tick_mob_gravity(
    mut query: Query<(&mut MobVelocity, &KinematicCharacterControllerOutput)>,
    gravity_acceleration: Res<GravityAcceleration>,
    time: Res<Time>,
) {
    let velocity_change = **gravity_acceleration * time.delta_seconds();

    for (mut velocity, output) in query.iter_mut() {
        if output.grounded {
            velocity.y = 0.0;
        } else {
            velocity.y += velocity_change;
        }
    }
}

pub fn resolve_mob_velocity(
    mut query: Query<(&MobVelocity, &mut KinematicCharacterController)>,
    terminal_velocity: Res<TerminalVelocity>,
) {
    for (velocity, mut controller) in query.iter_mut() {
        let velocity = velocity.clamp_length_max(**terminal_velocity);

        controller.translation = if let Some(translation) = controller.translation {
            Some(velocity + translation)
        } else {
            Some(velocity)
        };
    }
}
