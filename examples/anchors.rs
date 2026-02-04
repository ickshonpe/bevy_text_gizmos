//! Example demonstrating text anchors.

use bevy::color::palettes::css::{BLUE, GREEN, ORANGE, RED, YELLOW};
use bevy::prelude::*;
use bevy_text_gizmos::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, anchors)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn anchors(mut text_gizmos: Gizmos) {
    for (pos, label, anchor, color) in [
        (vec2(-350.0, 0.0), "left", vec2(-0.5, 0.0), RED),
        (vec2(350.0, 0.0), "right", vec2(0.5, 0.0), ORANGE),
        (vec2(0.0, 0.0), "center", Vec2::ZERO, YELLOW),
        (vec2(0.0, 220.0), "top", vec2(0.0, 0.5), GREEN),
        (vec2(0.0, -220.0), "bottom", vec2(0.0, -0.5), BLUE),
    ] {
        text_gizmos.text_2d(
            Isometry2d::from_translation(pos),
            "+",
            12.,
            Vec2::ZERO,
            Color::WHITE,
        );
        text_gizmos.text_2d(Isometry2d::from_translation(pos), label, 25., anchor, color);
    }
}
