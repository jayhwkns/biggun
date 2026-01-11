use bevy::prelude::*;

use crate::{
    environment::fish::{Fish, Hooked},
    player::hook::Hook,
};

/// **VERY** simple physics plugin. Responsible for moving objects with
/// velocity and making occasional collision checks.
pub struct BiggunPhysicsPlugin;

impl Plugin for BiggunPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (apply_velocity, check_hook_fish_collision));
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time<Fixed>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}

pub fn check_hook_fish_collision(
    hook_entity: Single<(&Transform, &mut Hook)>,
    mut fish_query: Query<(Entity, &Transform, &mut Fish, &mut Velocity)>,
    mut commands: Commands,
) {
    let (hook_transform, mut hook) = hook_entity.into_inner();
    if hook.hooked {
        return;
    }

    let hook_position = hook_transform.translation;
    for (entity, transform, mut fish, mut fish_velocity) in &mut fish_query {
        let fish_position = transform.translation;
        let dist = fish_position.distance(hook_position);
        if dist < hook.catch_radius {
            hook.hooked = true;
            fish.state.hooked = true;
            // We position fish manually while hooked
            fish_velocity.0 = Vec2::ZERO;
            // Mark as hooked for easy query
            commands.entity(entity).insert(Hooked);
            return;
        }
    }
}
