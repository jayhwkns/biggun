//! Code for input handling

// TODO: Each module should contain it's own input handling. Dissolve this module.

use bevy::prelude::*;

use crate::game_manager::state::StartGameEvent;

/// Handles input by triggering events
pub fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        commands.trigger(StartGameEvent);
    }
}
