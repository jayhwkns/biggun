//! A collection of all plugins and needed types for the game's `main` function

pub use crate::{
    config::Config,
    fish::{self, Fish},
    hook::{self, Hook},
    input, physics,
    scenes::BiggunScenePlugin,
    state::{self, CountdownTimer, Floor, GameState},
    ui::{self, MainMenuItem},
};
