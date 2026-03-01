use crate::keyboard_input_handler::KeyboardInputHandler;
use godot::{
    classes::{Camera3D, ICamera3D},
    obj::WithBaseField,
    prelude::{Base, GodotClass, Vector3, godot_api, godot_print},
};
use std::f32::consts;

// Step size of angular velocity changes for the orbital camera.
const ANGULAR_VELOCITY_STEP: f32 = 0.1;
// Maximum angular velocity for the orbital camera.
const MAX_ANGULAR_VELOCITY: Vector3 = Vector3::new(2.0, 2.0, 2.0);
// Extremely small value for cutoffs that are effectively zero
const EPSILON: f32 = 0.01;
// Decay factor for angular velocity when no input is detected
const MOVEMENT_DECAY: f32 = 0.75;

// ===== Definitions ===========================================================

/// Definition of our [`OrbitalCamera`] class, which inherits from [`Camera3D`].
#[derive(GodotClass)]
#[class(base=Camera3D)]
pub struct OrbitalCamera {
    /// The base [`Camera3D`] of our class that the [`OrbitalCamera`] is built upon.
    #[base]
    pub base: Base<Camera3D>,

    /// Vector representing the current (spherical) coordinates of the [`OrbitalCamera`].
    /// - `x`: Length of the vector (r), or distance from the origin.
    /// - `y`: Zenith angle (theta), from the Y axis to the vector.
    /// - `z`: Azimuthal angle (phi), from the X axis to the XZ projection.
    pub spherical_coordinates: Vector3,

    /// Vector representing the current (angular) velocity of the [`OrbitalCamera`].
    /// - `x`: Length of the vector (r), or distance from the origin.
    /// - `y`: Zenith angle (theta), from the Y axis to the vector.
    /// - `z`: Azimuthal angle (phi), from the X axis to the XZ projection.
    pub angular_velocity: Vector3,
}

// ===== Implementations =======================================================

/// Implementation of our [`OrbitalCamera`] class, which inherits from [`Camera3D`].
#[godot_api]
impl ICamera3D for OrbitalCamera {
    /// Called when the [`OrbitalCamera`] node is created.
    ///
    /// # Parameters:
    /// - `base`: The base [`Camera3D`] of our class that the [`OrbitalCamera`] is built upon.
    fn init(base: Base<Camera3D>) -> Self {
        godot_print!("OrbitalCamera created!");

        Self {
            base,
            spherical_coordinates: Self::cartesian_to_spherical(Vector3::new(0.0, 0.0, 2.0)),
            angular_velocity: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    /// Called every physics frame to update the state of the [`OrbitalCamera`].
    ///
    /// # Parameters:
    /// - `self`: The instance of the [`OrbitalCamera`] class.
    /// - `delta`: The time elapsed since the last physics frame, in seconds.
    fn physics_process(&mut self, delta: f32) {
        // Poll the WASDQE keys for movement input
        let movement_input: Vector3 = KeyboardInputHandler::poll_wasdqe_movement();

        if movement_input == Vector3::ZERO {
            // If there's no movement, apply damping
            self.angular_velocity *= MOVEMENT_DECAY;
        } else {
            self.angular_velocity += movement_input * ANGULAR_VELOCITY_STEP;
        }
        // Clamp the angular velocity to prevent it from growing indefinitely
        self.angular_velocity = self
            .angular_velocity
            .clamp(-MAX_ANGULAR_VELOCITY, MAX_ANGULAR_VELOCITY);

        // Update the camera's spherical coordinates based on angular velocity
        self.spherical_coordinates += self.angular_velocity * delta;
        // Clamp the spherical coordinate's length to prevent teleporting through the origin
        self.spherical_coordinates.x = self.spherical_coordinates.x.clamp(EPSILON, 10.0);
        // Clamp the spherical coordinate's zenith to avoid flipping over poles
        self.spherical_coordinates.y = self
            .spherical_coordinates
            .y
            .clamp(EPSILON, consts::PI - EPSILON);

        // Convert spherical coordinates to Cartesian coordinates
        let cartesian_coordinates: Vector3 =
            Self::spherical_to_cartesian(self.spherical_coordinates);
        // Set the camera's position to the new Cartesian coordinates
        self.base_mut().set_position(cartesian_coordinates);
        self.base_mut().look_at(Vector3::ZERO);
    }
}

/// Implementation of our [`OrbitalCamera`] class.
#[godot_api]
impl OrbitalCamera {
    /// Converts a vector of Cartesian coordinates to spherical coordinates.
    ///
    /// # Parameters:
    /// - `cartesian_coordinates`: The Cartesian coordinates to convert.
    ///
    /// # Returns:
    /// A vector of spherical coordinates:
    /// - `x`: Length of the vector (r), or distance from the origin.
    /// - `y`: Zenith angle (theta), from the Y axis to the vector.
    /// - `z`: Azimuthal angle (phi), from the X axis to the XZ projection.
    fn cartesian_to_spherical(cartesian_coordinates: Vector3) -> Vector3 {
        let x: f32 = cartesian_coordinates.x;
        let y: f32 = cartesian_coordinates.y;
        let z: f32 = cartesian_coordinates.z;

        // Construct and return our new vector
        Vector3::new(
            (x * x + y * y + z * z).sqrt(),
            (y / (x * x + y * y + z * z).sqrt()).acos(),
            z.atan2(x),
        )
    }

    /// Converts a vector of spherical coordinates to Cartesian coordinates.
    ///
    /// # Parameters:
    /// - `spherical_coordinates`: The spherical coordinates to convert.
    ///
    /// # Returns:
    /// A vector of Cartesian coordinates.
    fn spherical_to_cartesian(spherical_coordinates: Vector3) -> Vector3 {
        let r: f32 = spherical_coordinates.x;
        let theta: f32 = spherical_coordinates.y;
        let phi: f32 = spherical_coordinates.z;

        // Construct and return our new vector
        Vector3::new(
            r * theta.sin() * phi.cos(),
            r * theta.cos(),
            r * theta.sin() * phi.sin(),
        )
    }
}
