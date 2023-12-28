use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Q) {
            direction.x -= 1.0;  // Move left
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;  // Move right
        }

        if keyboard_input.pressed(KeyCode::Z) {
            direction.y += 1.0;  // Move up
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;  // Move down
        }

        // Normalize direction to have consistent movement speed in all directions
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        let movement_speed = 32.0; // Pixels per second
        let zoom_factor = 1. / transform.scale.x; // Assuming uniform scaling for x and y

        // Calculate the actual movement distance considering time and zoom factor
        let movement_distance = time.delta_seconds() * movement_speed * zoom_factor;

        // Update the translation based on direction and calculated movement distance
        transform.translation += direction * movement_distance;

        // Ensure the camera doesn't move in the Z direction
        transform.translation.z = 0.0;
    }
}
