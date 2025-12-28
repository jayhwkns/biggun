//! Contain and manipulate game state and score
use crate::{
    config::{Config, StageConfig},
    ui::ScoreDisplay,
};
use bevy::prelude::*;

/// Resource data pertaining to the state of the game
#[derive(Resource)]
pub struct State {
    /// The current number of fish in the game
    pub fish_count: u32,
    /// The stage we're on, used as an index for `stages` in config
    stage: usize,
    /// False when on the main menu
    pub started: bool,
    /// Score of the current stage
    score: u32,
}

impl State {
    /// Gets current stage or uses `config.sample_stage` as a fallback. Final
    /// stage in config gets repeated forever, but `stage` can still count up.
    pub fn cur_stage<'a>(&self, config: &'a Config) -> &'a StageConfig {
        if !self.started || config.stages.is_empty() {
            &config.sample_stage
        } else {
            &config.stages[self.stage.clamp(0, config.stages.len())]
        }
    }

    /// Increments stage by 1
    pub fn next_stage(&mut self) {
        self.stage += 1;
    }

    /// Resets state to intial (`default()`) values
    pub fn reset(&mut self) {
        *self = State::default();
    }

    pub fn add_to_score(&mut self, add: u32, display: Single<&mut Text, With<ScoreDisplay>>) {
        self.score += add;
        let mut text = display.into_inner();
        text.0 = format!("{:08}", self.score);
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            fish_count: 0,
            stage: 0,
            started: false,
        }
    }
}

#[derive(Component)]
#[require(Text)]
pub struct CountdownTimer {
    pub timer: Timer,
}

impl CountdownTimer {
    pub fn tick(single: Single<(&mut CountdownTimer, &mut Text)>, time: Res<Time>) {
        let (mut countdown, mut text) = single.into_inner();

        countdown.timer.tick(time.delta());

        let display_num = countdown.timer.remaining_secs() as i32;
        if display_num == 0 {
            text.0 = "".to_string();
        } else {
            text.0 = format!("{display_num}");
        }
    }
}

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Floor;
