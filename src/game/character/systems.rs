use bevy::prelude::*;
use super::components::*;

pub fn apply_input_to_desired_velocity(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut DesiredVelocity,
        &MoveSpeed,
        ),
        With<CharacterController>
    >
) {
    for (mut desired_velocity, move_speed) in &mut query {
        desired_velocity.linear.x = 0.0; desired_velocity.linear.y = 0.0;
        desired_velocity.linear.x -= f32::from(keys.pressed(KeyCode::KeyA)) * move_speed.value;
        desired_velocity.linear.x += f32::from(keys.pressed(KeyCode::KeyD)) * move_speed.value;
        desired_velocity.linear.y -= f32::from(keys.pressed(KeyCode::KeyS)) * move_speed.value;
        desired_velocity.linear.y += f32::from(keys.pressed(KeyCode::KeyW)) * move_speed.value;
    }

    
    
}

fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }
pub fn apply_desired_velocity(
    time: Res<Time>,
    mut query: Query<(
        &mut Velocity,
        &DesiredVelocity
    )>,
) {
    for (mut velocity, desired_velocity) in &mut query {
        velocity.linear.x = lerp(velocity.linear.x, desired_velocity.linear.x, time.delta_secs());
        velocity.linear.y = lerp(velocity.linear.y, desired_velocity.linear.y, time.delta_secs());
    }
}

pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(
        &Velocity,
        &mut Transform
    )>,
) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.linear.x * time.delta_secs();
        transform.translation.y += velocity.linear.y * time.delta_secs();
    }
}
