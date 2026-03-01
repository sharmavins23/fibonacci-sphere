use godot::prelude::{ExtensionLibrary, gdextension};

mod fibonacci_sphere;
mod keyboard_input_handler;
mod orbital_camera;
mod spotlight;

// ===== Definitions ===========================================================

/// Definition of the [`ExtensionLibrary`] loading our scripts into Godot.
struct RustExtensionLibrary;

// ===== Implementations =======================================================

/// Implementation of the [`ExtensionLibrary`] loading our scripts into Godot.
#[gdextension]
unsafe impl ExtensionLibrary for RustExtensionLibrary {}
