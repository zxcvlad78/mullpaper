use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_system)
        .run();
}

#[derive(Component)]
pub struct Player {
    pub name: String
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Player {name : "Player".to_string()});
}

pub fn player_system(query: Query<&Player>) {
    for player in query.iter() {
    }
}

