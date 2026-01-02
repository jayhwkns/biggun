//! Scene setup and transitioning

use crate::{
    config::Config,
    events::StartGameEvent,
    fish::{Fish, SpawnHandler},
    hook::{Guy, Hook, Rod},
    physics::Velocity,
    state,
    state::{CountdownTimer, Floor, GameState},
    ui::MainMenuItem,
};
use bevy::{
    core_pipeline::tonemapping::Tonemapping, post_process::bloom::Bloom, prelude::*, sprite::Anchor,
};
use bevy_prototype_lyon::prelude::*;

pub struct BiggunScenePlugin;

impl Plugin for BiggunScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_main_menu)
            .add_observer(load_game);
    }
}

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
        Guy,
    ));

    commands.spawn(SpawnHandler {
        timer: Timer::from_seconds(1.0, TimerMode::Once),
    });
}

/// Loads into the core gameplay loop **from main menu**.
fn load_game(
    _event: On<StartGameEvent>,
    mut commands: Commands,
    mut state: ResMut<GameState>,
    config: Res<Config>,
    menu_items: Query<Entity, With<MainMenuItem>>,
    floor: Single<&mut Transform, With<Floor>>,
    fish: Query<Entity, With<Fish>>,
    countdown_timer: Single<&mut CountdownTimer>,
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
    commands.spawn(
        ShapeBuilder::with(&line)
            .stroke((Color::WHITE, config.visuals.line_width))
            .build(),
    );

    state::stage_transition(config, state, commands, floor, fish, countdown_timer);
}
