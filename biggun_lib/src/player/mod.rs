use bevy::prelude::*;

pub(crate) mod fisherman;
pub(crate) mod hook;

/// Handles player actions.
pub struct BiggunPlayerPlugin;

/// Adds functionality or decoration to a player
#[derive(Component)]
#[relationship(relationship_target = PlayerOwns)]
pub struct OwnedByPlayer(pub Entity);

/// Contains all the entities decorating or adding functionality to a
/// certain player
#[derive(Component, Default)]
#[relationship_target(relationship = OwnedByPlayer)]
pub struct PlayerOwns(Vec<Entity>);

impl Plugin for BiggunPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hook::handle_input, fisherman::follow_hook))
            .add_systems(FixedUpdate, hook::check_extraction)
            .add_observer(hook::on_hook_event)
            .add_observer(hook::on_hook_lost);
    }
}
