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
