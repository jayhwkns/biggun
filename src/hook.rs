//! Movable hook and all related player components

use crate::config::Config;
use crate::fish::{self, Fish};
use crate::physics::Velocity;
use crate::state::State;
use crate::ui::ScoreDisplay;
use bevy::prelude::*;

/// Controllable hook when fishing
#[derive(Component)]
pub struct Hook {
    /// How fast you can move the hook horizontally
    pub speed: f32,
    /// How fast you can pull a fish upwards
    pub reel_speed: f32,
    /// How fast the hook will fall down in the water
    pub density: f32,
    /// True when a fish is on the hook
    pub hooked: bool,
    /// How close a fish's anchor (mouth) must be to the hook in order to catch
    pub catch_radius: f32,
}

/// Literally just a guy
#[derive(Component)]
#[require(Sprite)]
pub struct Guy;

/// Adjusts the hook's velocity according to user input
pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    hook_entity: Single<(&mut Velocity, &Transform, &Hook)>,
    guy: Single<(&mut Sprite, &Transform), With<Guy>>,
    hooked_fish: Option<Single<&Fish, With<fish::Hooked>>>,
    config: Res<Config>,
    state: Res<State>,
) {
    let (mut velocity, transform, hook) = hook_entity.into_inner();

    // Set initial horizontal velocity from keyboard input
    velocity.0 = Vec2::new(0., 0.);
    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.x -= hook.speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.x += hook.speed;
    }

    let mut vertical_resistance = 1.;

    let upper_bound = config.water_level;
    let lower_bound = config.water_level - state.cur_stage(&config).water_depth;
    velocity.y = if keyboard_input.pressed(KeyCode::Space) {
        hook.reel_speed / vertical_resistance
    } else {
        -hook.density
    };
    if transform.translation.y > upper_bound && velocity.y > 0. {
        velocity.y = 0.;
    } else if transform.translation.y < lower_bound && velocity.y < 0. {
        velocity.y = 0.;
    }

    // Have fish pull on hook if hooked
    if let Some(hooked_fish) = hooked_fish {
        // You can reel easier if you're not pulling in a direction
        if velocity.x.abs() > 0.5 {
            vertical_resistance = 4.;
        }
        velocity.x = hooked_fish.get_hook_velocity(hook, &velocity.0);
    }

    // Make guy face towards hook
    let (mut guy_sprite, guy_transform) = guy.into_inner();
    guy_sprite.flip_x = transform.translation.x < guy_transform.translation.x;
}

/// Extracts a hooked fish when the hook reaches the surface and adds to score
pub fn check_extraction(
    mut commands: Commands,
    hook_transform: Single<(&mut Transform, &mut Hook), Without<fish::Hooked>>,
    hooked_fish: Option<Single<(Entity, &Fish, &Transform), With<fish::Hooked>>>,
    mut state: ResMut<State>,
    score_display: Single<&mut Text, With<ScoreDisplay>>,
    config: Res<Config>,
) {
    // How close the hook mut be to the surface of the water to register the
    // extraction
    const SURFACE_DIST: f32 = 0.1;
    if let None = hooked_fish {
        return;
    }
    let (entity, hooked_fish, transform) = hooked_fish.unwrap().into_inner();
    let (mut hook_transform, mut hook) = hook_transform.into_inner();
    let mut score_display = score_display.into_inner();

    if transform.translation.y >= config.water_level - SURFACE_DIST {
        // Extraction has occured
        state.score += hooked_fish.get_score();
        score_display.0 = format!("{:08}", state.score);
        commands.entity(entity).despawn();
        hook_transform.translation = Vec3::new(0., config.water_level, 0.);
        hook.hooked = false;
    }
}

/// Checks all fish if we are able to hook onto them
pub fn check_fish(
    hook_entity: Single<(&Transform, &mut Hook)>,
    mut fish_query: Query<(Entity, &Transform, &mut Fish, &mut Velocity)>,
    mut commands: Commands,
) {
    let (hook_transform, mut hook) = hook_entity.into_inner();
    if hook.hooked {
        return;
    }

    let hook_position = hook_transform.translation;
    for (entity, transform, mut fish, mut fish_velocity) in &mut fish_query {
        let fish_position = transform.translation;
        let dist = fish_position.distance(hook_position);
        if dist < hook.catch_radius {
            hook.hooked = true;
            fish.state.hooked = true;
            // We position fish manually while hooked
            fish_velocity.0 = Vec2::ZERO;
            // Mark as hooked for easy query
            commands.entity(entity).insert(fish::Hooked);
            return;
        }
    }
}
