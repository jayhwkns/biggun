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
        .add_plugins((BiggunScenePlugin))
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(GameState::default())
        .insert_resource(Config::default())
        .add_systems(Startup, (ui::init_ui, ui::init_blinds, ui::init_main_menu))
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
