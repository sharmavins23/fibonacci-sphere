use godot::{
    classes::{DirectionalLight3D, INode3D, Node3D},
    obj::{Gd, NewAlloc, WithBaseField},
    prelude::{Base, GodotClass, Transform3D, Vector3, godot_api, godot_print},
};

// The distance of the spotlights from the origin
const SPOTLIGHT_DISTANCE: f32 = 10.0;

// ===== Definitions ===========================================================

/// Definition of our [`Spotlight`] class, which inherits from [`Node3D`].
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Spotlight {
    #[base]
    pub base: Base<Node3D>,
}

// ===== Implementations =======================================================

/// Implementation of our [`Spotlight`] class, which inherits from [`Node3D`].
#[godot_api]
impl INode3D for Spotlight {
    /// Called when the [`Spotlight`] node is created.
    ///
    /// # Parameters:
    /// - `base`: The base [`Node3D`] of our class that the [`Spotlight`] is built upon.
    fn init(base: Base<Node3D>) -> Self {
        godot_print!("Spotlight created!");

        Self { base }
    }

    /// Called when the [`Spotlight`] node is ready in the scene.
    ///
    /// Places 6 spotlights at each of the cardinal directions facing the origin.
    ///
    /// # Parameters:
    /// - `self`: The instance of the [`Spotlight`] class.
    fn ready(&mut self) {
        // Set up all spotlights as children
        let spotlight_positions: Vec<Vector3> = vec![
            Vector3::UP,
            Vector3::DOWN,
            Vector3::LEFT,
            Vector3::RIGHT,
            Vector3::FORWARD,
            Vector3::BACK,
        ];

        // For each spotlight position...
        spotlight_positions.iter().for_each(|position: &Vector3| {
            let mut spotlight: Gd<DirectionalLight3D> = DirectionalLight3D::new_alloc();

            // Reset the transform to ensure no prior rotation or scaling interferes
            spotlight.set_transform(Transform3D::IDENTITY);

            // Set the position of the spotlight
            let mut transform: Transform3D = spotlight.get_transform();
            transform.origin = *position * SPOTLIGHT_DISTANCE;

            // Make the spotlight look at the origin (Vector3::ZERO)
            transform = Transform3D::looking_at(&transform, Vector3::ZERO, Vector3::UP, false);

            // Apply the transform
            spotlight.set_transform(transform);

            // Add the spotlight as a child of the base node
            self.base_mut().add_child(Some(&spotlight));
        });
    }
}
