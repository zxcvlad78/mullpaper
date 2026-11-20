use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component, Default)]
pub struct CharacterInput {
    pub direction: Vec2
} 

#[derive(Component, Default)]
pub struct Velocity {
    pub linear: Vec2,
    pub angular: f32
} 

#[derive(Component, Default)]
pub struct DesiredVelocity {
    pub linear: Vec2,
    pub angular: f32
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
