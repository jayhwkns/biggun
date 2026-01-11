use bevy::prelude::*;

pub(crate) mod config;
pub(crate) mod scenes;
pub(crate) mod state;

/// Game management for biggun. Scene transition, state management, etc.
pub struct BiggunGameManagerPlugin;

impl Plugin for BiggunGameManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(state::stage_transition)
            .add_systems(Update, (state::CountdownTimer::tick, state::handle_input))
            .add_systems(Startup, scenes::load_main_menu)
            .add_observer(scenes::load_game);
    }
}
