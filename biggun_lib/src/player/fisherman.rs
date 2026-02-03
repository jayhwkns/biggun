//! Visuals for fisherman and the rod
use super::{PlayerOwns, hook::Hook};
use crate::{
    environment::fish::{Fish, FishExtractedEvent},
    game_manager::{config::Config, scenes::SceneVolatile},
    prelude::GameState,
    utils::ui::ScoreDisplay,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

/// Fisherman visual representing the player
#[derive(Component)]
#[require(PlayerOwns)]
pub struct Fisherman;

/// Visual rod that follows the hook
#[derive(Component)]
#[require(SceneVolatile)]
pub struct Rod;

#[derive(Component)]
#[require(SceneVolatile)]
pub struct FishingLine;

/// Makes fisherman follow the hook visually
pub fn follow_hook(
    guy: Single<(&mut Sprite, &Transform), With<Fisherman>>,
    rod: Single<&mut Transform, (With<Rod>, Without<Hook>, Without<Fisherman>)>,
    line: Single<&mut Shape, With<FishingLine>>,
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

pub fn on_extraction(
    event: On<FishExtractedEvent>,
    mut commands: Commands,
    mut state: ResMut<GameState>,
    fish_query: Query<&Fish>,
    mut score_display: Single<&mut Text, With<ScoreDisplay>>,
) {
    let Ok(fish) = fish_query.get(event.fish) else {
        warn!(
            "Extracted fish {} does not exist in query. Was it removed too early?",
            event.fish
        );
        return;
    };
    state.score += fish.get_score();
    score_display.0 = format!("SCORE {:08}", state.score);
    state.fish_count -= 1;
    commands.entity(event.fish).despawn();
}
