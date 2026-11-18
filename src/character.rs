use bevy::prelude::*;
pub mod components;
pub mod systems;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_desired_velocity_to_controller, apply_gravity_to_controller),
        );
    }
}