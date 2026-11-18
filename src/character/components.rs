use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct DesiredVelocity { vector: Vec2 } 


#[derive(Component)]
pub struct MoveSpeed { value: f32 }
impl Default for MoveSpeed {
    fn default() -> Self {
        Self { value: 100.0 }
    }
}

#[derive(Component)]
pub struct MoveInput { pub vector: Vec2 }
impl Default for MoveInput {
    fn default() -> Self {
        Self { vector: Vec2 { x: 0.0, y: 0.0 } }
    }
}

