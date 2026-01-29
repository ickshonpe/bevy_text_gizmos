use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextGizmosPlugin))
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
    let position = Vec3::new(-5.0, 0.0, 0.0);
    let rotation = Quat::from_rotation_y(0.5);
    let isometry = Isometry3d::new(position, rotation);
    text_gizmos.text_3d(isometry, "Hello, text gizmos", 2.0, Color::WHITE);
}
