use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Snake {
    segments: Vec<Position>,
    direction: Direction,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //Camera sjust
    commands.spawn(Camera2dBundle::default());

    //Snake head created
    let triangulo = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::Y * 50.0,
        Vec2::new(-50.0, -50.0),
        Vec2::new(50.0, -50.0),
    )));

    let color = Color::hsl(0.0, 1.0, 0.5);

    commands.spawn(MaterialMesh2dBundle {
        mesh: triangulo.clone(),
        material: materials.add(color),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
