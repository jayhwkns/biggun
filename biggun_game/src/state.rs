//! Contain and manipulate game state and score
use std::time::Duration;

use crate::{
    config::{Config, StageConfig},
    fish::{self, Fish},
    hook::Hook,
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
    pub score: u32,
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
}

impl Default for State {
    fn default() -> Self {
        State {
            fish_count: 0,
            stage: 0,
            started: false,
            score: 0,
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

    /// Resets the timer with a new time
    pub fn reset_timer(&mut self, new_time: Duration) {
        self.timer = Timer::new(new_time, TimerMode::Once);
    }
}

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Floor;

/// Transitions to `state`'s current stage
pub fn stage_transition(
    config: Res<Config>,
    mut state: ResMut<State>,
    mut commands: Commands,
    floor: Single<&mut Transform, With<Floor>>,
    fish: Query<Entity, With<Fish>>,
    countdown_timer: Single<&mut CountdownTimer>,
) {
    let stage = state.cur_stage(&config);

    fish::despawn_all(fish, commands, state);
    let mut floor_transform = floor.into_inner();
    floor_transform.translation.y = config.water_level - stage.water_depth;

    countdown_timer.into_inner().reset_timer(stage.time);
}
