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
        desired_velocity.vector.x = 0.0; desired_velocity.vector.y = 0.0;
        desired_velocity.vector.x -= f32::from(keys.pressed(KeyCode::KeyA)) * move_speed.value;
        desired_velocity.vector.x += f32::from(keys.pressed(KeyCode::KeyD)) * move_speed.value;
        desired_velocity.vector.y -= f32::from(keys.pressed(KeyCode::KeyW)) * move_speed.value;
        desired_velocity.vector.y += f32::from(keys.pressed(KeyCode::KeyS)) * move_speed.value;
        println!("{}", desired_velocity.vector.to_string());
    }
}

// fn apply_desired_velocity(
//     time: Res<Time>,
//     mut query: Query<(
//         &DesiredVelocity,
//         &MoveSpeed,
//         &mut KinematicCharacterController,
//     )>,
// ) {
//     for (desired_velocity, speed, mut controller) in &mut query {
//         let desired_translation = desired_velocity.0 * speed.0 * time.delta_secs();
//         controller.translation = Some(desired_translation);
//     }
// }

// fn apply_gravity(
//     time: Res<Time>,
//     mut query: Query<&mut CharacterController, With<>>,
// ) {
//     let gravity_step = Vec2::new(0.0, -9.81 * 100.0) * time.delta_secs();

//     for mut controller in &mut query {
//         if let Some(ref mut translation) = controller.translation {
//             translation.y += gravity_step.y;
//         } else {
//             controller.translation = Some(gravity_step);
//         }
//     }
// }