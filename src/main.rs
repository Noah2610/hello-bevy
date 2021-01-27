extern crate bevy;

mod components;
mod plugins;
mod systems;

use bevy::app::App;
use bevy::DefaultPlugins;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::GreeterPlugin)
        .run();
}
