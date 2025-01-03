use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(EditorPlugin::new().in_separate_window())
        .add_plugins(EditorPlugin::new().in_new_window(Window::default()))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(
            Plane3d::new(Vec3::Y, Vec2::new(2.5, 2.5)).mesh(),
        ))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::from_size(Vec3::ONE)))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0)));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
