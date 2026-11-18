use bevy::prelude::*;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Velocity {
    pub linear: Vec2,
    pub angular: f32
} 
impl Default for Velocity {
    fn default() -> Self {
        Self {
            linear: Vec2::default(),
            angular: 0.0
        }
    }
}

#[derive(Component)]
pub struct DesiredVelocity {
    pub linear: Vec2,
    pub angular: f32
} 
impl Default for DesiredVelocity {
    fn default() -> Self {
        Self {
            linear: Vec2::default(),
            angular: 0.0
        }
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
