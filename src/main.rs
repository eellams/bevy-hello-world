use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::math::primitives::Rectangle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

#[derive(Component)]
struct RotatingCube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the rotating rectangle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(100.0, 100.0))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(0.2, 0.4, 0.8))),
            transform: Transform::default(),
            ..default()
        },
        RotatingCube,
    ));
}

fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds());
    }
}
