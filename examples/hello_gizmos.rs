//! Basic example displaying a message using text gizmos

use bevy::prelude::*;
use bevy_text_gizmos::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, hello_world)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
}

fn hello_world(mut text_gizmos: Gizmos) {
    text_gizmos.text(
        Isometry3d::new(Vec3::new(-5.0, 1.0, 0.0), Quat::from_rotation_y(0.6)),
        "Hello, text gizmos",
        1.,
        Color::WHITE,
    );
}
