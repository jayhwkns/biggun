use bevy::prelude::*;
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
        .add_plugins((
            BiggunPlayerPlugin,
            BiggunGameManagerPlugin,
            BiggunPhysicsPlugin,
            BiggunEnvironmentPlugin,
        ))
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(GameState::default())
        .insert_resource(Config::default())
        .run();
}
