# bevy_text_gizmos

Text gizmo implementation for Bevy using stroke fonts.

## Usage

Add the depedency to the dependencies section of your project's `cargo.toml`:
```
bevy_text_gizmos = "0.3"
```

Then just import the `TextGizmos` trait and draw some text gizmos.

```rust
use bevy::prelude::*;
use bevy_text_gizmos::TextGizmos;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .add_systems(Update, |mut gizmos: Gizmos| {
            gizmos.text_2d(
                Isometry2d::IDENTITY,
                "text gizmo",
                25.0,
                Vec2::ZERO,
                Color::WHITE,
            );
        })
        .run();
}
```

## Examples

```
cargo run --example hello_gizmos
```

![hello_text_gizmos](/screenshots/hello_text_gizmos.png)

```
cargo run --example anchors
```

![anchors.png](/screenshots/anchors.png)

```
cargo run --example stress_text
```

![example.png](/screenshots/example.png)

```
cargo run --example all_glyphs
```

![all_glyphs.png](/screenshots/all_glyphs.png)


```
cargo run --example 3d_text_gizmos
```

![3d_text_gizmos.png](/screenshots/3d_text_gizmos.png)