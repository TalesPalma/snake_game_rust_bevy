use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

pub fn spawn_apple_radom_locatint(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    // Gera números aleatórios para a posição x e y
    let random_position_x: f32 = rng.gen();
    let random_position_y: f32 = rng.gen();

    // Ajusta os valores para a faixa desejada
    let adjusted_random_position_x = (random_position_x - 0.5) * 1000.0;
    let adjusted_random_position_y = (random_position_y - 0.5) * 1000.0;

    // Apple spawn
    let circle = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));
    let color = Color::hsl(0.0, 1.0, 0.2);

    commands.spawn(MaterialMesh2dBundle {
        mesh: circle.clone().into(),
        material: materials.add(color),
        transform: Transform::from_xyz(adjusted_random_position_x, adjusted_random_position_y, 0.0),
        ..Default::default()
    });
}
