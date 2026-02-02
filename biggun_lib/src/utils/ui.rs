//! User Interface such as scoring HUD and main menu
use bevy::prelude::*;

#[derive(Component)]
#[require(Text)]
pub struct ScoreDisplay;
