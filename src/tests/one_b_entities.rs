use bevy::prelude::*;

use crate::game::character;


pub struct OneBEntitiesPlugin;
impl Plugin for OneBEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn);
    }
}

fn spawn(
    mut commands: Commands,
) {

    commands.spawn(character::bundles::CharacterBundle::default());
}