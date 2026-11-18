use bevy::prelude::*;
pub mod character;
use character::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(character::CharacterPlugin)
        .add_systems(Startup, spawn)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        components::MoveSpeed::default(),
        components::DesiredVelocity::default(),
    ));
}


