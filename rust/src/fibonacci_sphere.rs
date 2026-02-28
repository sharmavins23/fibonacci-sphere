use godot::{
    classes::{INode3D, Node3D},
    prelude::{Array, Base, GodotClass, Vector3, godot_api, godot_print},
};

// ===== Definitions ===========================================================

/// Definition of our [`FibonacciSphere`] class, which inherits from [`Node3D`].
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct FibonacciSphere {
    /// The base [`Node3D`] of our class that the [`FibonacciSphere`] is built upon.
    #[base]
    pub base: Base<Node3D>,

    /// The points of the [`FibonacciSphere`].
    #[allow(unused)]
    pub points: Array<Vector3>,
}

// ===== Implementations =======================================================

/// Implementation of our [`FibonacciSphere`] class.
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
}
