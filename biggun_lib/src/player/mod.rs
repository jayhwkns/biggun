use bevy::prelude::*;

pub(crate) mod fisherman;
pub(crate) mod hook;

/// Handles player actions.
pub struct BiggunPlayerPlugin;

impl Plugin for BiggunPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hook::handle_input, fisherman::follow_hook))
            .add_systems(FixedUpdate, hook::check_extraction)
            .add_observer(hook::on_hook_event);
    }
}
