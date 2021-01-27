extern crate bevy;

mod components;
mod systems;

use bevy::app::App;
use bevy::ecs::IntoSystem;
use bevy::DefaultPlugins;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::add_people.system())
        .add_system(systems::hello_world.system())
        .add_system(systems::greet_people.system())
        .run();
}
