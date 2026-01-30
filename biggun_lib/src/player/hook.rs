//! Movable hook and all related player components

use crate::{
    environment::fish::{self, Fish, Hooked},
    game_manager::{config::Config, state::GameState},
    physics::Velocity,
    utils::ui::ScoreDisplay,
};

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

impl Hook {
    pub fn start_pos(config: &Config) -> Vec3 {
        Vec3::new(0.0, config.water_level, 0.0)
    }
}

/// Adjusts the hook's velocity according to user input
pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    hook: Single<(&mut Velocity, &Transform, &Hook)>,
    hooked_fish: Option<Single<&Fish, With<fish::Hooked>>>,
    config: Res<Config>,
    state: Res<GameState>,
) {
    let (mut velocity, transform, hook) = hook.into_inner();

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
    // Have fish pull on hook if hooked
    if let Some(hooked_fish) = hooked_fish {
        // You can reel easier if you're not pulling in a direction
        if velocity.x.abs() > 0.5 {
            vertical_resistance = 4.;
        }
        velocity.x = hooked_fish.get_hook_velocity(hook, &velocity.0);
    }
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
}

/// Extracts a hooked fish when the hook reaches the surface and adds to score
pub fn check_extraction(
    mut commands: Commands,
    hook_transform: Single<(&mut Transform, &mut Hook), Without<Hooked>>,
    hooked_fish: Single<(Entity, &Fish), With<Hooked>>,
    mut state: ResMut<GameState>,
    score_display: Single<&mut Text, With<ScoreDisplay>>,
    config: Res<Config>,
) {
    // How close the hook mut be to the surface of the water to register the
    // extraction
    const SURFACE_DIST: f32 = 0.1;
    let (entity, hooked_fish) = hooked_fish.into_inner();
    let (mut hook_transform, mut hook) = hook_transform.into_inner();
    let mut score_display = score_display.into_inner();

    if hook_transform.translation.y >= config.water_level - SURFACE_DIST {
        // Extraction has occured
        state.score += hooked_fish.get_score();
        score_display.0 = format!("SCORE {:08}", state.score);
        commands.entity(entity).despawn();
        hook_transform.translation = Vec3::new(0., config.water_level, 0.);
        hook.hooked = false;
    }
}

/// Occurs between a hook and a fish when a hook touches a fish.
#[derive(Event)]
pub struct HookEvent {
    pub hook_entity: Entity,
    pub fish_entity: Entity,
}

/// Connects fish and hook
pub fn on_hook_event(
    event: On<HookEvent>,
    mut commands: Commands,
    mut fish_query: Query<(&mut Fish, &mut Transform, &mut Velocity)>,
    mut hook_query: Query<&mut Hook>,
) {
    let event = event.event();
    commands
        .entity(event.hook_entity)
        .add_child(event.fish_entity);
    commands.entity(event.fish_entity).insert(Hooked);

    if let Ok((mut fish, mut fish_transform, mut fish_velocity)) =
        fish_query.get_mut(event.fish_entity)
    {
        fish_velocity.0 = Vec2::ZERO;
        fish_transform.translation = Vec3::ZERO;
        fish.state.hooked = true;
    } else {
        warn!(
            "Hook event called on a non-existent Fish: {}",
            event.fish_entity
        );
    }

    if let Ok(mut hook) = hook_query.get_mut(event.hook_entity) {
        hook.hooked = true;
    } else {
        warn!(
            "Hook event called on a non-existent Hook: {}",
            event.hook_entity
        );
    }
}
