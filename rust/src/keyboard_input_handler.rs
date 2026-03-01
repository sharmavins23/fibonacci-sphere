use godot::{
    classes::Input,
    global::Key,
    obj::{Gd, Singleton},
    prelude::Vector3,
};

// ===== Definitions ===========================================================

/// Definition of our [`KeyboardInputHandler`] class.
pub struct KeyboardInputHandler;

// ===== Implementations =======================================================

/// Implementation of our [`KeyboardInputHandler`] class.
impl KeyboardInputHandler {
    /// Checks if a specific key is pressed.
    ///
    /// # Parameters:
    /// - `key`: The key to check for being pressed.
    fn is_key_pressed(key: Key) -> bool {
        let input: Gd<Input> = Input::singleton();
        input.is_key_pressed(key)
    }

    /// Polls the WASDQE keys for movement input, and returns a normalized
    /// movement vector corresponding to the keys being pressed.
    ///
    /// Note that since the orbital camera moves in spherical coordinates,
    /// the movement input is returned in spherical coordinates, as well.
    ///
    /// # Returns:
    /// A [`Vector3`] representing the movement direction.
    pub fn poll_wasdqe_movement() -> Vector3 {
        // Mapping of WASD keys to their corresponding movement directions
        let actions: [(Key, Vector3); 6] = [
            // Movement based on vector length (r)
            (Key::Q, Vector3::new(1.0, 0.0, 0.0)),
            (Key::E, Vector3::new(-1.0, 0.0, 0.0)),
            // Movement based on zenith angle (theta)
            (Key::S, Vector3::new(0.0, 1.0, 0.0)),
            (Key::W, Vector3::new(0.0, -1.0, 0.0)),
            // Movement based on azimuthal angle (phi)
            (Key::A, Vector3::new(0.0, 0.0, 1.0)),
            (Key::D, Vector3::new(0.0, 0.0, -1.0)),
        ];

        // Compute the sum of all input direction vectors
        let direction: Vector3 = actions
            .iter()
            .filter(|(key, _)| Self::is_key_pressed(*key))
            .fold(
                Vector3::ZERO,
                |sum_vector: Vector3, (_, direction): &(Key, Vector3)| sum_vector + *direction,
            );

        // Normalize the resulting direction vector
        if direction != Vector3::ZERO {
            direction.normalized()
        } else {
            Vector3::ZERO
        }
    }

    /// Polls the Z and X keys for point modification input, and returns an
    /// integer representing the modification direction.
    ///
    /// - Returns `-1` if Z is pressed (decrease point count).
    /// - Returns `1` if X is pressed (increase point count).
    /// - Returns `0` if neither or both are pressed (no change).
    pub fn poll_zx_point_modification() -> i32 {
        match (Self::is_key_pressed(Key::Z), Self::is_key_pressed(Key::X)) {
            (true, false) => -1, // Z is pressed, decrease point count
            (false, true) => 1,  // X is pressed, increase point count
            _ => 0,              // No change if both or neither are pressed
        }
    }
}
