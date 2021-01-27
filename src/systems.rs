use crate::components;
use bevy::ecs::{Commands, Query, With};

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

pub fn greet_people(query: Query<&components::Name, With<components::Person>>) {
    for name in query.iter() {
        println!("Hello {}", name.0);
    }
}
