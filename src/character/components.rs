use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct DesiredVelocity { pub vector: Vec2 } 
impl Default for DesiredVelocity {
    fn default() -> Self {
        Self { vector: Vec2::default() }
    }
}

#[derive(Component)]
pub struct MoveSpeed { pub value: f32 }
impl Default for MoveSpeed {
    fn default() -> Self {
        Self { value: 100.0 }
    }
}

#[derive(Component)]
pub struct CharacterController;
