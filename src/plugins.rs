use crate::resources;
use crate::systems;
use bevy::app::{AppBuilder, Plugin};
use bevy::core::Timer;
use bevy::ecs::IntoSystem;

pub struct GreeterPlugin;

impl Plugin for GreeterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(resources::GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(systems::add_people.system())
            // .add_system(systems::hello_world.system())
            .add_system(systems::greet_people.system());
    }
}
