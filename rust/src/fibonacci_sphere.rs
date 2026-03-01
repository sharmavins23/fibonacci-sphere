use godot::{
    classes::{INode3D, MeshInstance3D, Node, Node3D, SphereMesh, StandardMaterial3D},
    obj::{Gd, NewAlloc, NewGd, WithBaseField},
    prelude::{Array, Base, Color, GodotClass, Transform3D, Vector3, godot_api, godot_print},
};
use std::f32::consts;

use crate::keyboard_input_handler::KeyboardInputHandler;

// Minimum number of points.
const MIN_POINTS: i32 = 2;
// Maximum number of points.
const MAX_POINTS: i32 = 5000;
// Size of each individual rendered point.
const POINT_SIZE: f32 = 0.02;

// ===== Definitions ===========================================================

/// Definition of our [`FibonacciSphere`] class, which inherits from [`Node3D`].
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct FibonacciSphere {
    /// The base [`Node3D`] of our class that the [`FibonacciSphere`] is built upon.
    #[base]
    pub base: Base<Node3D>,

    /// The points of the [`FibonacciSphere`].
    pub points: Array<Vector3>,
}

// ===== Implementations =======================================================

/// Implementation of our [`FibonacciSphere`] class, which inherits from [`INode3D`].
#[godot_api]
impl INode3D for FibonacciSphere {
    /// Called when the [`FibonacciSphere`] node is created.
    ///
    /// # Parameters:
    /// - `base`: The base [`Node3D`] of our class that the [`FibonacciSphere`] is built upon.
    fn init(base: Base<Node3D>) -> Self {
        godot_print!("FibonacciSphere created!");

        Self {
            base,
            points: Array::new(),
        }
    }

    /// Called when the [`FibonacciSphere`] node is ready in the scene.
    ///
    /// # Parameters:
    /// - `self`: The instance of the [`FibonacciSphere`] class.
    fn ready(&mut self) {
        godot_print!("FibonacciSphere is ready in the scene!");

        self.generate_points(MIN_POINTS);
    }

    /// Called every physics frame to update the state of the [`FibonacciSphere`].
    ///
    /// # Parameters:
    /// - `self`: The instance of the [`FibonacciSphere`] class.
    /// - `_delta`: The time elapsed since the last physics frame.
    fn physics_process(&mut self, _delta: f32) {
        // Poll the ZX keys for point increment/decrement
        let point_delta: i32 = KeyboardInputHandler::poll_zx_point_modification();
        if point_delta != 0 {
            let current_point_count: i32 = self.points.len() as i32;
            let num_points: i32 = (current_point_count + point_delta)
                // Clamp to reasonable values to prevent overflow or freezing
                .max(MIN_POINTS)
                .min(MAX_POINTS);
            self.generate_points(num_points);
        }
    }
}

/// Implementation of our [`FibonacciSphere`] class.
#[godot_api]
impl FibonacciSphere {
    /// Clears all previously rendered points from the [`FibonacciSphere`].
    ///
    /// # Parameters:
    /// - `self`: The instance of the [`FibonacciSphere`] class.
    fn clear_rendered_points(&mut self) {
        let child_nodes: Array<Gd<Node>> = self.base().get_children();

        for child in child_nodes.iter_shared() {
            self.base_mut().remove_child(Some(&child));
            child.free(); // Free the child node to prevent memory leaks
        }
    }

    /// Generates the points of the [`FibonacciSphere`].
    ///
    /// # Parameters:
    /// - `self`: The instance of the [`FibonacciSphere`] class.
    /// - `num_points`: The number of points to generate on the sphere.
    #[func]
    pub fn generate_points(&mut self, num_points: i32) {
        self.points.clear();
        self.clear_rendered_points();

        // Pre-compute the golden ratio
        // Note: std::f32::consts::PHI is not yet available in stable Rust
        let phi: f32 = (1.0 + 5.0_f32.sqrt()) / 2.0; // The golden ratio

        // Iteratively calculate all points
        for i in 0..num_points {
            let i: f32 = i as f32;
            let num_points: f32 = num_points as f32;

            // y goes from 1 to -1
            let y: f32 = 1.0 - (i / (num_points - 1.0)) * 2.0;

            // Compute the radius of the planar circle
            // This corresponds to the hypotenuse for calculating x, z
            let radius: f32 = (1.0 - y * y).sqrt();

            // Angular position of the point in the x-z plane
            let theta: f32 = 2.0 * consts::PI * i / phi;

            // Compute cartesian coordinates of x, z
            let x: f32 = theta.cos() * radius;
            let z: f32 = theta.sin() * radius;

            // Add the point to the array
            self.points.push(Vector3::new(x, y, z));
        }

        // Render all points on screen!
        self.render_points();
    }

    /// Renders the points of the [`FibonacciSphere`] as small spheres in the scene.
    ///
    /// # Parameters:
    /// - `self`: The instance of the [`FibonacciSphere`] class.
    fn render_points(&mut self) {
        let points: Array<Vector3> = self.points.duplicate_shallow();

        for point in points.iter_shared() {
            // Map the point's x, y, z coordinates to r, g, b
            let (r, g, b) = (
                (point.x + 1.0) / 2.0, // Map x from [-1, 1] to [0, 1]
                (point.y + 1.0) / 2.0, // Map y from [-1, 1] to [0, 1]
                (point.z + 1.0) / 2.0, // Map z from [-1, 1] to [0, 1]
            );

            // Create a new material for each point
            let mut material = StandardMaterial3D::new_gd();
            material.set_albedo(Color::from_rgb(r, g, b));

            // Create a new SphereMesh for each point
            let mut sphere_mesh: Gd<SphereMesh> = SphereMesh::new_gd();
            sphere_mesh.set_radius(POINT_SIZE);
            sphere_mesh.set_height(POINT_SIZE);
            sphere_mesh.set_material(Some(&material));

            // Create a new MeshInstance3D for each point
            // ! These have to be manually freed via free() after usage
            let mut mesh_instance_3d: Gd<MeshInstance3D> = MeshInstance3D::new_alloc();
            mesh_instance_3d.set_mesh(Some(&sphere_mesh));

            // Set the position of the sphere
            let transform: Transform3D = Transform3D::new(
                // Use the identity basis (no rotation)
                Transform3D::IDENTITY.basis,
                // Set the origin to the point's position
                point,
            );
            mesh_instance_3d.set_transform(transform);

            // Add the sphere to the scene
            self.base_mut().add_child(Some(&mesh_instance_3d));
        }
    }
}
