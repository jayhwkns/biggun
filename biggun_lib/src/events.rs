//! This module contains all Biggun events

// TODO: Dissolve module

use bevy::prelude::*;

#[derive(Event)]
pub struct StartGameEvent;

#[derive(Event)]
pub struct NextStageEvent;
