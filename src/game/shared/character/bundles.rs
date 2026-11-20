use bevy::prelude::*;
use super::components::*;

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    character:        Character,
    velocity:         Velocity,
    desired_velocity: DesiredVelocity,
    pub move_speed:   MoveSpeed,
    pub transform:    Transform,
    pub sprite:       Sprite
}
