use bevy::prelude::*;
pub mod components;
pub mod systems;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            systems::apply_input_to_desired_velocity
        );
    }
}

