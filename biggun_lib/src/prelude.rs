//! A collection of all plugins and needed types for the game's `main` function

pub use crate::{
    environment::fish::Fish,
    game_manager::{
        config::Config,
        scenes::BiggunScenePlugin,
        state::{CountdownTimer, Floor, GameState, GameStateManagerPlugin},
    },
    input, physics,
    player::hook::Hook,
};
