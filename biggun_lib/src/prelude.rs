//! A collection of all plugins and needed types for the game's `main` function

pub use crate::{
    environment::BiggunEnvironmentPlugin,
    game_manager::{BiggunGameManagerPlugin, config::Config, state::GameState},
    physics::BiggunPhysicsPlugin,
    player::BiggunPlayerPlugin,
};
