//! Code for input handling

use bevy::prelude::*;

use crate::events::StartGameEvent;

/// Handles input by triggering events
pub fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        commands.trigger(StartGameEvent);
    }
}
