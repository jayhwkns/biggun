//! Contain and manipulate game state and score
use std::time::Duration;

use crate::environment::fish::Fish;

use super::config::{Config, StageConfig};

use bevy::prelude::*;

#[derive(Event)]
pub struct StartGameEvent;

#[derive(Event)]
pub struct NextStageEvent;

#[derive(Event)]
pub struct GameOverEvent;

/// Resource data pertaining to the state of the game
#[derive(Resource)]
pub struct GameState {
    /// The current number of fish in the game
    pub fish_count: u32,
    /// The stage we're on, used as an index for `stages` in config
    stage: usize,
    /// False when on the main menu
    pub started: bool,
    /// Score of the current stage
    pub score: u32,
}

impl GameState {
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
        *self = GameState::default();
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
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
    _event: On<NextStageEvent>,
    config: Res<Config>,
    state: Res<GameState>,
    mut commands: Commands,
    floor: Single<&mut Transform, With<Floor>>,
    fish: Query<Entity, With<Fish>>,
    countdown_timer: Single<&mut CountdownTimer>,
) {
    let stage = state.cur_stage(&config);

    fish.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
    let mut floor_transform = floor.into_inner();
    floor_transform.translation.y = config.water_level - stage.water_depth;

    countdown_timer.into_inner().reset_timer(stage.time);
}

/// Handles state controls such as pausing and starting the game.
pub fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        commands.trigger(StartGameEvent);
    }
}
