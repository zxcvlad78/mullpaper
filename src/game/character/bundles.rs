use bevy::prelude::*;
use super::components::*;

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    transform: Transform,
    velocity: Velocity,
    desired_velocity: DesiredVelocity,
    move_speed: MoveSpeed,
    pub sprite: Sprite
}
