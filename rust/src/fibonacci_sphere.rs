use godot::{
    classes::{INode3D, MeshInstance3D, Node, Node3D, SphereMesh, StandardMaterial3D},
    obj::{Gd, NewAlloc, NewGd, WithBaseField},
    prelude::{Array, Base, Color, GodotClass, Transform3D, Vector3, godot_api, godot_print},
};
use std::f32::consts;

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

    /// Re-usable [`SphereMesh`] for the fixed points.
    pub sphere_mesh: Gd<SphereMesh>,
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
        godot_print!("FibonacciSphere creating!");

        // Created the material for the fixed points
        let mut material = StandardMaterial3D::new_gd();
        material.set_albedo(Color::from_rgb(1.0, 0.0, 0.0));

        // Create the fixed point
        let mut sphere_mesh: Gd<SphereMesh> = SphereMesh::new_gd();
        sphere_mesh.set_radius(POINT_SIZE);
        sphere_mesh.set_height(POINT_SIZE);
        sphere_mesh.set_material(Some(&material));

        Self {
            base,
            points: Array::new(),
            sphere_mesh,
        }
    }

    /// Called when the [`FibonacciSphere`] node is ready in the scene.
    ///
    /// # Parameters:
    /// - `self`: The instance of the [`FibonacciSphere`] class.
    fn ready(&mut self) {
        godot_print!("FibonacciSphere is ready in the scene!");

        self.generate_points(1000);
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

            godot_print!("Generated point {}: ({}, {}, {})", i, x, y, z);
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
            // Create a new MeshInstance3D for each point
            // ! These have to be manually freed via free() after usage
            let mut mesh_instance_3d: Gd<MeshInstance3D> = MeshInstance3D::new_alloc();
            mesh_instance_3d.set_mesh(Some(&self.sphere_mesh));

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
