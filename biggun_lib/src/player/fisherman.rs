//! Visuals for fisherman and the rod
use super::hook::Hook;
use crate::game_manager::{config::Config, scenes::SceneVolatile};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

/// Fisherman visual
#[derive(Component)]
#[require(Sprite)]
pub struct Fisherman;

/// Visual rod that follows the hook
#[derive(Component)]
#[require(Sprite, SceneVolatile)]
pub struct Rod;

#[derive(Component)]
#[require(Shape, SceneVolatile)]
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
