//! This module contains all Biggun events

use bevy::prelude::*;

#[derive(Event)]
pub struct StartGameEvent;

#[derive(Event)]
pub struct NextStageEvent;
