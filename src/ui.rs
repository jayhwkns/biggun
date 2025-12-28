//! User Interface such as scoring HUD and main menu
use crate::{config::Config, state::CountdownTimer};
use bevy::{prelude::*, sprite::Anchor};

#[derive(Component)]
#[require(Text)]
pub struct ScoreDisplay;

/// Despawns on game start
#[derive(Component)]
pub struct MainMenuItem;

pub fn init_ui(mut commands: Commands, assets: Res<AssetServer>, config: Res<Config>) {
    let font = assets.load("kodemono.ttf");
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
}

pub fn init_main_menu(mut commands: Commands, assets: Res<AssetServer>, config: Res<Config>) {
    let font = assets.load("kodemono.ttf");
    let visuals = &config.visuals;

    // Logo
    let image_node = ImageNode::new(assets.load("logo.png"));
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

/// Initializes the black boxes that cover the out of bounds area in world space
pub fn init_blinds(mut commands: Commands, config: Res<Config>) {
    let visuals = &config.visuals;

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
}
