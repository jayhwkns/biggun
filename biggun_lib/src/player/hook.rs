//! Movable hook and all related player components

use std::f32::consts::PI;

use crate::{
    environment::fish::{self, Fish},
    game_manager::{config::Config, state::GameState},
    physics::Velocity,
    utils::ui::ScoreDisplay,
};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

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

/// Literally just a guy
#[derive(Component)]
#[require(Sprite)]
pub struct Guy;

/// Visual rod that follows the hook
#[derive(Component)]
#[require(Sprite)]
pub struct Rod;

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

/// Makes fisherman follow the hook visually
pub fn guy_follow_hook(
    guy: Single<(&mut Sprite, &Transform), With<Guy>>,
    rod: Single<&mut Transform, (With<Rod>, Without<Hook>, Without<Guy>)>,
    line: Single<&mut Shape>,
    hook_transform: Single<&Transform, With<Hook>>,
    config: Res<Config>,
) {
    // How far the hook must be for the rod to be fully extended
    const ROD_EXTEND: f32 = 64.;

    let hook_transform = hook_transform.into_inner();

    // Make guy face towards hook
    let (mut guy_sprite, guy_transform) = guy.into_inner();
    guy_sprite.flip_x = hook_transform.translation.x < guy_transform.translation.x;

    // Rotate rod towards hook
    let mut rod_transform = rod.into_inner();
    let hook_rod_dist =
        (hook_transform.translation.x - rod_transform.translation.x).clamp(-ROD_EXTEND, ROD_EXTEND);
    let rod_rot = (PI / 2.) + (PI / 2.) * ops::sin((PI * hook_rod_dist) / (2. * ROD_EXTEND));
    rod_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0., rod_rot, 0.);

    // Update fishing line
    let local_offset = Transform::from_xyz(-32., 32., 0.).to_matrix();
    let new_line = (
        // Get end of rod
        Transform::from_matrix(rod_transform.to_matrix() * local_offset)
            .translation
            .xy(),
        // Get hook position
        hook_transform.translation.xy() + Vec2::new(0., 8.),
    );
    let new_line = shapes::Line(new_line.0, new_line.1);
    *line.into_inner() = ShapeBuilder::with(&new_line)
        .stroke((Color::WHITE, config.visuals.line_width))
        .build();
}

/// Extracts a hooked fish when the hook reaches the surface and adds to score
pub fn check_extraction(
    mut commands: Commands,
    hook_transform: Single<(&mut Transform, &mut Hook), Without<fish::Hooked>>,
    hooked_fish: Single<(Entity, &Fish, &Transform), With<fish::Hooked>>,
    mut state: ResMut<GameState>,
    score_display: Single<&mut Text, With<ScoreDisplay>>,
    config: Res<Config>,
) {
    // How close the hook mut be to the surface of the water to register the
    // extraction
    const SURFACE_DIST: f32 = 0.1;
    let (entity, hooked_fish, transform) = hooked_fish.into_inner();
    let (mut hook_transform, mut hook) = hook_transform.into_inner();
    let mut score_display = score_display.into_inner();

    if transform.translation.y >= config.water_level - SURFACE_DIST {
        // Extraction has occured
        state.score += hooked_fish.get_score();
        score_display.0 = format!("SCORE {:08}", state.score);
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
