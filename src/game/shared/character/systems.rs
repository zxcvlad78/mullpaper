use bevy::prelude::*;
use super::components::*;
use crate::game::shared::network::components::*;

pub fn read_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut InputDirection>
) {
    for mut input_direction in &mut query {
        input_direction.up    = keys.pressed(KeyCode::KeyW);
        input_direction.down  = keys.pressed(KeyCode::KeyS);
        input_direction.left  = keys.pressed(KeyCode::KeyA);
        input_direction.right = keys.pressed(KeyCode::KeyD);
    }
}

pub fn apply_input_to_desired_velocity(
    mut query: Query<(
        &mut DesiredVelocity,
        &InputDirection,
        &MoveSpeed,
        ),
        With<CharacterController>
    >
) {
    for (mut desired_velocity, input_direction, move_speed) in &mut query {
        desired_velocity.linear.x = 0.0; desired_velocity.linear.y = 0.0;
        desired_velocity.linear.y += f32::from(input_direction.up)    * move_speed.value;
        desired_velocity.linear.y -= f32::from(input_direction.down)  * move_speed.value;
        desired_velocity.linear.x -= f32::from(input_direction.left)  * move_speed.value;
        desired_velocity.linear.x += f32::from(input_direction.right) * move_speed.value;
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
        if velocity.linear != Vec2::ZERO {
            transform.translation.x += velocity.linear.x * time.delta_secs();
            transform.translation.y += velocity.linear.y * time.delta_secs();
        }
        if velocity.angular != 0.0 {
            transform.rotation.x += velocity.angular * time.delta_secs();;
        }
    }
}
