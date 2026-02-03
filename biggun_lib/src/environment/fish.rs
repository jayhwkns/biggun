use crate::{physics::Velocity, player::hook::HookLostEvent, utils::layers::Layer};
use bevy::{math::FloatExt, prelude::*, sprite::Anchor};

use crate::{
    game_manager::{config::Config, scenes::SceneVolatile, state::GameState},
    player::hook::{Hook, HookedObjects},
    utils::units::{Inches, Ounces},
};

#[derive(Clone)]
enum Direction {
    Left = -1,
    Neutral = 0,
    Right = 1,
}

impl Direction {
    fn different(&self) -> (Direction, Direction) {
        match self {
            Self::Left => (Self::Neutral, Self::Right),
            Self::Neutral => (Self::Left, Self::Right),
            Self::Right => (Self::Left, Self::Neutral),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Heading {
    Left,
    Right,
}

#[derive(Component)]
#[require(Transform, Velocity, SceneVolatile)]
pub struct Fish {
    stats: FishStats,
    species: &'static Species,
    pub state: FishState,
}

/// Partially redundant with Fish.state.hooked, so that we can easily query for
/// a single hooked fish.
#[derive(Component)]
#[relationship(relationship_target = HookedObjects)]
pub struct HookedBy(pub Entity);

impl Fish {
    fn extra_strength(&self) -> f32 {
        self.stats.strength - self.species.base_stats.strength
    }

    fn extra_energy(&self) -> f32 {
        self.stats.energy - self.species.base_stats.energy
    }

    pub fn get_speed(&self) -> f32 {
        self.species.base_speed + self.extra_strength() * self.species.strength_to_speed
    }

    pub fn get_bobbing(&self) -> f32 {
        self.extra_strength() * self.species.strength_to_bobbing
    }

    /// Gets the total velocity of the hook and fish.
    ///
    /// When in the same direction, total velocity = direction * (hook-speed
    /// + `fish.strength_to_hook` * `fish.stats.strength`).
    ///
    /// When opposing, the velocity is 0 so it is possible to pull any fish
    pub fn get_hook_velocity(&self, hook: &Hook, hook_velocity: &Vec2) -> f32 {
        let hook_speed = hook_velocity.x.abs();
        let hook_direction = if hook_speed == 0. {
            0.
        } else {
            hook_velocity.x.signum()
        };
        let fish_direction = self.state.pulling.clone() as i8 as f32;
        if fish_direction == 0. {
            // Don't change if fish isn't pulling
            hook_velocity.x
        } else if hook_direction == 0. || fish_direction == hook_direction {
            // Fish has influence with player is in same direction
            fish_direction * (hook_speed + self.species.strength_to_hook * self.stats.strength)
        } else {
            // Player and fish are opposing, do not move
            0.
        }
    }

    pub fn get_frequency(&self) -> f32 {
        self.extra_energy() * self.species.energy_to_frequency * std::f32::consts::PI * 2.
    }

    /// Gets the scoring value of a fish
    pub fn get_score(&self) -> u32 {
        self.species.base_score + self.stats.weight.0 * self.stats.length.0
    }
}

/// Statistics to determine how a fish of a species behaves
#[derive(Clone)]
pub struct FishStats {
    /// How difficult it is to reel. Primary factor
    /// In scoring. Measured in ounces.
    pub weight: Ounces,
    /// Multiplied by weight in scoring. Measured in inches.
    pub length: Inches,
    /// How difficult it is to wrangle the fish horizontally.
    pub strength: f32,
    /// Determines how frequently the fish will change direction
    pub energy: f32,
    /// How far deep the fish will spawn
    pub depth: Inches,
    /// Which way the fish is facing
    heading: Heading,
}

pub struct FishState {
    pub hooked: bool,
    age: f32,
    pulling: Direction,
    pub timer: Timer,
}

impl Default for FishState {
    fn default() -> Self {
        FishState {
            hooked: false,
            age: 0.,
            pulling: Direction::Neutral,
            timer: Timer::from_seconds(0., TimerMode::Once),
        }
    }
}

impl Fish {
    /// Creates a new instance of a fish with randomized stats in between
    /// base stats and max stats
    fn new(species: &'static Species) -> Fish {
        let w_weight = rand::random::<f32>();
        let w_length = rand::random::<f32>();
        let w_strength = rand::random::<f32>();
        let w_energy = rand::random::<f32>();
        let w_depth = rand::random::<f32>();
        let roll_heading = rand::random::<f32>();
        Fish {
            stats: FishStats {
                weight: species
                    .base_stats
                    .weight
                    .lerp(&species.max_stats.weight, w_weight),
                length: species
                    .base_stats
                    .length
                    .lerp(&species.max_stats.length, w_length),
                strength: species
                    .base_stats
                    .strength
                    .lerp(species.max_stats.strength, w_strength),
                energy: species
                    .base_stats
                    .energy
                    .lerp(species.max_stats.energy, w_energy),
                depth: species
                    .base_stats
                    .depth
                    .lerp(&species.max_stats.depth, w_depth),
                heading: if roll_heading >= 0.5 {
                    Heading::Left
                } else {
                    Heading::Right
                },
            },
            species: &species,
            state: FishState::default(),
        }
    }

    /// Spawns a new fish into Bevy
    fn spawn_new(
        species: &'static Species,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        config: &Res<Config>,
        state: &GameState,
    ) {
        let mut fish = Fish::new(&species);

        // If under floor, re-roll depth with a floor as a boundary
        let floor_depth = state.cur_stage(config).water_depth as u32;
        if fish.stats.depth.0 > floor_depth {
            let w_depth = rand::random::<f32>();
            fish.stats.depth = species.base_stats.depth.lerp(&Inches(floor_depth), w_depth)
        }
        let y = config.water_level - fish.stats.depth.0 as f32;

        let facing_left = fish.stats.heading == Heading::Left;
        let invert = if facing_left { -1. } else { 1. };
        let speed = fish.get_speed();
        // 1in -> 1px
        let scale = fish.stats.length.0 as f32 / fish.species.img_size.x;

        commands.spawn((
            fish,
            Sprite {
                image: asset_server.load(species.img_path),
                flip_x: facing_left,
                ..default()
            },
            Transform {
                translation: Vec3::new(config.game_width * -invert, y, Layer::FISH),
                scale: Vec3::ONE * scale,
                ..default()
            },
            if facing_left {
                Anchor::CENTER_LEFT
            } else {
                Anchor::CENTER_RIGHT
            },
            Velocity(Vec2::new(speed * invert, 0.)),
        ));
    }
}

/// Representation of a specimen of fish. Should be constructed at start and be
/// static throughout the game.
pub struct Species {
    /// Path to sprite image from `assets/`
    img_path: &'static str,
    img_size: Vec2,
    /// Fish struct containing minimum stats
    base_stats: FishStats,
    /// Fish struct containing maximum stats
    max_stats: FishStats,
    /// How much each additional unit of strength from minimum should be turned
    /// into horizontal speed
    strength_to_speed: f32,
    /// The base amount of score obtained when catching
    base_score: u32,
    base_speed: f32,
    /// How much each additional unit of strength from minimum should be turned
    /// into sinusoidal vertical speed (bobbing)
    strength_to_bobbing: f32,
    /// How much each additional unit of strength from minimum should be turned
    /// into velocity applied the hook
    strength_to_hook: f32,
    /// How much additional unit of energy from minimum should be turned into
    /// bobbing frequency
    energy_to_frequency: f32,
    /// The maximum amount of time it can take to turn around
    struggle_time: f32,
}

impl Species {
    pub const MIN_STRUGGLE: f32 = 0.08;
    pub const MAX_STRUGGLE: f32 = 5.;
    pub const BASS: Species = Species {
        img_path: "bass.png",
        img_size: Vec2::new(32., 16.),
        base_stats: FishStats {
            weight: Ounces(6),
            length: Inches(10),
            strength: 5.,
            energy: 1.,
            depth: Inches::from_ft_ins(2, 6),
            heading: Heading::Left,
        },
        max_stats: FishStats {
            weight: Ounces::from_lbs_ozs(10, 5),
            length: Inches(29),
            strength: 10.,
            energy: 1.5,
            depth: Inches::from_ft_ins(40, 0),
            heading: Heading::Left,
        },
        strength_to_speed: 3.,
        base_score: 100,
        base_speed: 3.,
        strength_to_bobbing: 2.5,
        strength_to_hook: 10.25,
        energy_to_frequency: 0.6,
        struggle_time: 2.,
    };
}

#[derive(Component)]
pub struct SpawnHandler {
    pub timer: Timer,
}

#[derive(Event)]
pub struct FishEscapedEvent {
    pub entity: Entity,
}

pub fn handle_spawn(
    commands: Commands,
    mut spawn_handler: Single<&mut SpawnHandler>,
    config: Res<Config>,
    mut state: ResMut<GameState>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let fish_count = state.fish_count;
    let max_fish = state.cur_stage(&config).max_fish;
    if spawn_handler.timer.is_finished() && fish_count < max_fish {
        let new_interval: f32 = 5.0 * rand::random::<f32>() + 1.0;
        spawn_handler.timer = Timer::from_seconds(new_interval, TimerMode::Once);
        state.fish_count += 1;
        Fish::spawn_new(
            &Species::BASS,
            commands,
            asset_server,
            &config,
            &state.into_inner(),
        );
    }
    spawn_handler.timer.tick(time.delta());
}

/// Moves and despawns fish
pub fn update_fish(
    fish_query: Query<(
        Entity,
        &mut Fish,
        &GlobalTransform,
        &mut Velocity,
        Option<&HookedBy>,
    )>,
    mut commands: Commands,
    config: Res<Config>,
    time: Res<Time>,
) {
    const ESCAPE_LENIENCE: f32 = 1.;
    for (entity, mut fish, transform, mut velocity, hooked_by) in fish_query {
        // Despawn escaped fish
        if transform.translation().x.abs() > config.game_width + ESCAPE_LENIENCE {
            commands.trigger(FishEscapedEvent { entity });
            continue;
        }

        if let Some(_) = hooked_by {
            continue;
        }

        fish.state.age += time.delta_secs();
        *velocity = Velocity(Vec2::new(
            velocity.x,
            fish.get_bobbing() * ops::cos(fish.state.age * fish.get_frequency()),
        ));
    }
}

/// Allows fish to periodically change direction using random timers
pub fn struggle(fish_query: Single<&mut Fish, With<HookedBy>>, time: Res<Time>) {
    let mut fish = fish_query.into_inner();
    if fish.state.timer.is_finished() {
        let new_interval: f32 = (fish.species.struggle_time * rand::random::<f32>())
            .clamp(Species::MIN_STRUGGLE, Species::MAX_STRUGGLE);
        fish.state.timer = Timer::from_seconds(new_interval, TimerMode::Once);
        // Change direction
        let diff = fish.state.pulling.different();
        fish.state.pulling = if rand::random::<f32>() >= 0.5 {
            diff.0
        } else {
            diff.1
        };
    }
    fish.state.timer.tick(time.delta());
}

pub fn on_fish_escape(
    event: On<FishEscapedEvent>,
    mut commands: Commands,
    fish_query: Query<Option<&HookedBy>, With<Fish>>,
    mut state: ResMut<GameState>,
) {
    let entity = event.event().entity;
    commands.entity(entity).despawn();
    state.fish_count -= 1;

    if let Ok(Some(_)) = fish_query.get(entity) {
        commands.trigger(HookLostEvent);
    }
}
