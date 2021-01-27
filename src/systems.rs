use crate::components;
use crate::resources::GreetTimer;
use bevy::core::Time;
use bevy::ecs::{Commands, Query, Res, ResMut, With};

pub fn add_people(commands: &mut Commands) {
    use components::{Name, Person};

    commands
        .spawn((Person, Name(String::from("Noah Ro"))))
        .spawn((Person, Name(String::from("Walter White"))))
        .spawn((Person, Name(String::from("Jimmy McGill"))));
}

pub fn hello_world() {
    println!("Hello World!");
}

pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&components::Name, With<components::Person>>,
) {
    timer.0.tick(time.delta_seconds());
    if !timer.0.just_finished() {
        return;
    }

    for name in query.iter() {
        println!("Hello {}", name.0);
    }
}
