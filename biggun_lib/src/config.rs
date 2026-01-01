//! Configuration data for the game. Should take the place of any would-be
//! hard-coded constants.

use crate::fish::Species;
use bevy::prelude::*;
use std::time::Duration;

/// Configuration for each stage or level
pub struct StageConfig {
    /// The score needed to pass this stage
    pub target_score: u32,
    /// The maximum y distance from `water_level` where fish can spawn.
    /// If greater than a species minimum depth, do not spawn that fish.
    /// Else clamp the species maximum depth to this value to distribute evenly.
    pub water_depth: f32,
    /// The number of fish allowed during this stage
    pub max_fish: u32,
    /// The types of fish that can spawn during this stage
    pub species: Vec<Species>,
    /// How much time to give the player to complete the stage
    pub time: Duration,
}

/// Configuration for game visuals (font sizes, colors, etc.)
pub struct VisualConfig {
    pub score_font_size: f32,
    pub score_padding: f32,
    /// Opacity of box covering out-of-bounds region
    pub blinds_opacity: f32,
    pub info_font_size: f32,
    /// Width of the fishing line in world scale
    pub line_width: f32,
}

#[derive(Resource)]
pub struct Config {
    /// How far out the boundaries are from the center of the screen (x=0)
    pub game_width: f32,
    /// The y coordinate in world-space where the water is located
    pub water_level: f32,
    /// The configuration for each stage. Ordered. Upon reaching the end of
    /// this vector, the last stage is repeated forever.
    pub stages: Vec<StageConfig>,
    /// The stage that is displayed behind the main menu or as a fallback
    /// when `stages` is empty
    pub sample_stage: StageConfig,
    /// Font sizes, colors, etc.
    pub visuals: VisualConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_width: 180.,
            water_level: 50.,
            stages: vec![StageConfig {
                target_score: 100,
                water_depth: 150.,
                max_fish: 5,
                species: vec![Species::BASS],
                time: Duration::from_secs_f32(60.),
            }],
            sample_stage: StageConfig {
                target_score: 0,
                water_depth: 200.,
                max_fish: 20,
                species: vec![Species::BASS],
                time: Duration::from_secs_f32(0.),
            },
            visuals: VisualConfig {
                score_font_size: 32.,
                score_padding: 5.,
                blinds_opacity: 0.8,
                info_font_size: 18.,
                line_width: 0.5,
            },
        }
    }
}

// TODO: Configurable... config. Use toml?
