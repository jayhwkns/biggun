//! Scene setup and transitioning

use crate::{
    environment::fish::SpawnHandler,
    physics::Velocity,
    player::{
        fisherman::{Fisherman, FishingLine, Rod},
        hook::Hook,
    },
    utils::ui::{MainMenuItem, ScoreDisplay},
};

use bevy::{
    core_pipeline::tonemapping::Tonemapping, post_process::bloom::Bloom, prelude::*, sprite::Anchor,
};

use super::{
    config::Config,
    state::{self, CountdownTimer, GameState, NextStageEvent, StartGameEvent},
};
use bevy_prototype_lyon::prelude::*;

/// Loads into the main menu
pub fn load_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>) {
    // Camera
    commands.spawn((
        Camera2d,
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.25,
            ..default()
        },
        Transform {
            scale: Vec3::new(0.5, 0.5, 1.),
            ..default()
        },
    ));

    // Water
    commands.spawn((
        Sprite {
            image: asset_server.load("water_surface.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.,
            },
            custom_size: Some(Vec2::new(2048., 16.)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, config.water_level, 2.0),
            ..default()
        },
    ));

    // Floor
    commands.spawn((
        Sprite {
            image: asset_server.load("floor.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.,
            },
            custom_size: Some(Vec2::new(2048., 16.)),
            ..default()
        },
        Anchor::TOP_CENTER,
        Transform {
            translation: Vec3::new(
                0.0,
                config.water_level - config.sample_stage.water_depth,
                2.0,
            ),
            ..default()
        },
        state::Floor,
    ));

    // Boat
    commands.spawn((
        Sprite {
            image: asset_server.load("boat.png"),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 62., 1.),
            ..default()
        },
    ));

    // Fisherman
    commands.spawn((
        Sprite {
            image: asset_server.load("fisherman.png"),
            ..default()
        },
        Transform {
            translation: Vec3::new(0., config.water_level + 24., 0.),
            ..default()
        },
        Fisherman,
    ));

    commands.spawn(SpawnHandler {
        timer: Timer::from_seconds(1.0, TimerMode::Once),
    });

    // UI
    let font = asset_server.load("kodemono.ttf");
    let visuals = &config.visuals;

    // Logo
    let image_node = ImageNode::new(asset_server.load("logo.png"));
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            height: percent(100. / 3.),
            aspect_ratio: Some(2.),
            ..default()
        },
        image_node,
        MainMenuItem,
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            top: percent(50),
            ..default()
        },
        Text::new("press [ENTER] to start"),
        TextFont::from(font.clone()).with_font_size(visuals.info_font_size),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        MainMenuItem,
    ));
}

/// Loads into the core gameplay loop **from main menu**.
pub fn load_game(
    _event: On<StartGameEvent>,
    mut commands: Commands,
    mut state: ResMut<GameState>,
    config: Res<Config>,
    menu_items: Query<Entity, With<MainMenuItem>>,
    asset_server: Res<AssetServer>,
) {
    for item in menu_items {
        commands.entity(item).despawn();
    }
    state.started = true;

    // Hook
    commands.spawn((
        Sprite {
            image: asset_server.load("hook.png"),
            ..default()
        },
        Transform {
            translation: Hook::start_pos(&config),
            ..default()
        },
        Hook {
            speed: 35.0,
            reel_speed: 60.0,
            density: 10.0,
            hooked: false,
            catch_radius: 8.,
        },
        Velocity(Vec2::ZERO),
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("rod.png"),
            ..default()
        },
        Anchor::BOTTOM_RIGHT,
        Transform {
            translation: Vec3::new(0., config.water_level + 10., 1.),
            ..default()
        },
        Rod,
    ));

    let line = shapes::Line(Vec2::ZERO, Vec2::new(0.0, -100.0));
    commands.spawn((
        ShapeBuilder::with(&line)
            .stroke((Color::WHITE, config.visuals.line_width))
            .build(),
        FishingLine,
    ));

    // UI
    let font = asset_server.load("kodemono.ttf");
    let visuals = &config.visuals;

    // Score
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(visuals.score_padding),
            left: px(visuals.score_padding),
            ..default()
        },
        Text::new("SCORE 00000000"),
        TextFont::from(font.clone()).with_font_size(visuals.score_font_size),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        ScoreDisplay,
    ));

    // Target score
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(visuals.score_padding),
            right: px(visuals.score_padding),
            ..default()
        },
        Text::new("00000000 TARGET"),
        TextFont::from(font.clone()).with_font_size(visuals.score_font_size),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
    ));

    // Countdown
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(visuals.score_padding),
            width: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            ..default()
        },
        Text::new("30"),
        TextFont::from(font.clone()).with_font_size(visuals.score_font_size),
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        CountdownTimer {
            timer: Timer::new(config.sample_stage.time, TimerMode::Once),
        },
    ));

    // Left blind
    commands.spawn((
        Sprite::from_color(Color::srgba(0., 0., 0., visuals.blinds_opacity), Vec2::ONE),
        Anchor::CENTER_RIGHT,
        Transform {
            translation: Vec3::new(-config.game_width, 0., 100.),
            scale: Vec3::new(10000., 10000., 1.),
            ..default()
        },
    ));

    // Right blind
    commands.spawn((
        Sprite::from_color(Color::srgba(0., 0., 0., visuals.blinds_opacity), Vec2::ONE),
        Anchor::CENTER_LEFT,
        Transform {
            translation: Vec3::new(config.game_width, 0., 100.),
            scale: Vec3::new(10000., 10000., 1.),
            ..default()
        },
    ));

    // Go to next stage
    commands.trigger(NextStageEvent);
}
