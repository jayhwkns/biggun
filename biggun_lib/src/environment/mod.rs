use bevy::prelude::*;

pub(crate) mod fish;

/// Handles the non-player elements of the environment. Notably fish.
pub struct BiggunEnvironmentPlugin;

impl Plugin for BiggunEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fish::update_fish, fish::struggle))
            .add_systems(FixedUpdate, fish::handle_spawn)
            .add_observer(fish::on_fish_escape);
    }
}
