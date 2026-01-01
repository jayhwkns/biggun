use bevy::{
    core_pipeline::tonemapping::Tonemapping, post_process::bloom::Bloom, prelude::*, sprite::Anchor,
};
use bevy_prototype_lyon::prelude::*;
use biggun_lib::prelude::*;

const BG_COLOR: Color = Color::srgb(0.01, 0.01, 0.01);

fn main() {
    App::new()
        // Official bevy plugins
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(), // Use pixel perfect sprites
        ))
        // External plugins
        .add_plugins(ShapePlugin)
        // Custom, biggun specific, plugins
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(GameState::default())
        .insert_resource(Config::default())
        .add_observer(on_game_start)
        .add_systems(
            Startup,
            (setup, ui::init_ui, ui::init_blinds, ui::init_main_menu),
        )
        .add_systems(
            Update,
            (
                hook::handle_input,
                input::handle_input,
                fish::update_fish,
                fish::struggle,
                state::CountdownTimer::tick,
                hook::guy_follow_hook,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                physics::move_objects,
                fish::handle_spawn,
                hook::check_fish,
                hook::check_extraction,
            ),
        )
        .run();
}

fn on_game_start(
    _event: On<GameStarted>,
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
        hook::Hook {
            speed: 35.0,
            reel_speed: 60.0,
            density: 10.0,
            hooked: false,
            catch_radius: 8.,
        },
        physics::Velocity(Vec2::ZERO),
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
        hook::Rod,
    ));
    let line = shapes::Line(Vec2::ZERO, Vec2::new(0.0, -100.0));
    commands.spawn(
        ShapeBuilder::with(&line)
            .stroke((Color::WHITE, config.visuals.line_width))
            .build(),
    );

    state::stage_transition(config, state, commands, floor, fish, countdown_timer);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, config: Res<Config>) {
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
        hook::Guy,
    ));

    commands.spawn(fish::SpawnHandler {
        timer: Timer::from_seconds(1.0, TimerMode::Once),
    });
}
