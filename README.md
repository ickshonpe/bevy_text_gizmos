# bevy_text_gizmos

Text gizmo implementation for Bevy using stroke fonts.

## Usage

Add the depedency to the dependencies section of your project's `cargo.toml`:
```
bevy_text_gizmos = "0.1"
```

Then just import the `TextGizmos` trait: 

```rust
use bevy::prelude::*;
use bevy_text_gizmos::TextGizmos;

fn system(mut gizmos: Gizmos) {    
    gizmos.text(Isometry3d::IDENTITY, "text gizmo", 25.0, Color::WHITE);
}
```
