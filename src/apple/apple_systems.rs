use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};
use rand::Rng;

#[derive(Component)]
pub struct Apple {
    pub position: Vec2,
}

pub fn spawn_apple_radom_locatint(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let width = windows.single().width();
    let height = windows.single().height();

    let mut rng = rand::thread_rng();

    // Ajusta os valores para a faixa desejada
    let random_position_x = rng.gen_range(-width / 2.0..width / 2.0);
    let random_position_y: f32 = rng.gen_range(-height / 2.0..height / 2.0);
    // Apple spawn
    apple_spawn(
        meshes,
        commands,
        materials,
        random_position_x,
        random_position_y,
    );
}

pub fn apple_spawn(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position_x: f32,
    position_y: f32,
) {
    let circle = Mesh2dHandle(meshes.add(Circle { radius: 5.0 }));
    let color = Color::hsl(0.0, 1.0, 0.2);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: circle.clone().into(),
            material: materials.add(color),
            transform: Transform::from_xyz(position_x, position_y, 0.0),
            ..Default::default()
        })
        .insert(Apple {
            position: Vec2::new(position_x, position_y),
        });
}
