use bevy::prelude::*;
mod character;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_system)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Character);
}

pub fn player_system(query: Query<&Character>) {
    for player in query.iter() {
        
    }
}

