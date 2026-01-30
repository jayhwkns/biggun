use bevy::prelude::*;

use crate::{
    environment::fish::Fish,
    player::hook::{Hook, HookEvent},
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

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity(Vec2::ZERO)
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time<Fixed>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}

pub fn check_hook_fish_collision(
    hook_entity: Single<(Entity, &Transform, &Hook)>,
    mut fish_query: Query<(Entity, &Transform), With<Fish>>,
    mut commands: Commands,
) {
    let (hook_entity, hook_transform, hook) = hook_entity.into_inner();
    if hook.hooked {
        return;
    }

    let hook_position = hook_transform.translation;
    for (fish_entity, fish_transform) in &mut fish_query {
        let fish_position = fish_transform.translation;
        let dist = fish_position.distance(hook_position);
        if dist < hook.catch_radius {
            commands.trigger(HookEvent {
                hook_entity,
                fish_entity,
            });
            return;
        }
    }
}
