use godot::prelude::{ExtensionLibrary, gdextension};

// ===== Definitions ===========================================================

/// Definition of the [`ExtensionLibrary`] loading our scripts into Godot.
struct RustExtensionLibrary;

// ===== Implementations =======================================================

/// Implementation of the [`ExtensionLibrary`] loading our scripts into Godot.
#[gdextension]
unsafe impl ExtensionLibrary for RustExtensionLibrary {}
