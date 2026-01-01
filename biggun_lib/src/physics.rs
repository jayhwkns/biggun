use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

pub fn move_objects(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time<Fixed>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}
