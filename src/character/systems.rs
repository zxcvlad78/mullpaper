use bevy::prelude::*;
use super::components::*;

fn apply_desired_velocity_to_controller(
    time: Res<Time>,
    mut query: Query<(
        &DesiredVelocity,
        &Speed,
        &mut KinematicCharacterController,
    )>,
) {
    for (desired_velocity, speed, mut controller) in &mut query {
        let desired_translation = desired_velocity.0 * speed.0 * time.delta_secs();
        controller.translation = Some(desired_translation);
    }
}

fn apply_gravity_to_controller(
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController, With<Speed>>,
) {
    let gravity_step = Vec2::new(0.0, -9.81 * 100.0) * time.delta_secs();

    for mut controller in &mut query {
        if let Some(ref mut translation) = controller.translation {
            translation.y += gravity_step.y;
        } else {
            controller.translation = Some(gravity_step);
        }
    }
}