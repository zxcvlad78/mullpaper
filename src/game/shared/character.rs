use bevy::prelude::*;
pub mod components;
pub mod systems;
pub mod bundles;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                systems::read_input,
                systems::apply_input_to_desired_velocity,
                systems::apply_desired_velocity,
                systems::apply_velocity
            ).chain()
        );
    }
}

