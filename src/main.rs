use bevy::prelude::*;
mod game;
use game::*;

mod tests;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_plugins(tests::one_b_entities::OneBEntitiesPlugin)
        .run();
}

