mod apple;
mod game_behavior;
mod snake;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use snake::snake_systems::{
    keyboard_input, move_snake_controller_system, move_snake_in_screen, DirectionMoviment,
    Position, Snake,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_snake_controller_system)
        .add_systems(Update, move_snake_in_screen)
        .add_systems(Update, keyboard_input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //Camera ajuste
    commands.spawn(Camera2dBundle::default());

    //Snake body
    let triangulo = Mesh2dHandle(meshes.add(Rectangle::new(20.0, 20.0)));
    let color = Color::hsl(0.0, 1.0, 0.2);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: triangulo.clone(),
            material: materials.add(color),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Snake {
            segments: vec![
                Position { x: 0.0, y: 0.0 },
                Position { x: 2.0, y: 0.0 },
                Position { x: 20.0, y: 0.0 },
            ],
            direction: DirectionMoviment::Up,
        });
}
