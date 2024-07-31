use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    apple::apple_systems::{spawn_apple_radom_locatint, Apple},
    snake::snake_systems::*,
};

pub fn behavior_game_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&Window, With<PrimaryWindow>>,
    snake_query: Query<(&Transform, &Snake)>,
    apple_query: Query<(Entity, &Transform, &Apple)>,
) {
    for (snake_transform, snake) in snake_query.iter() {
        for (apple_entity, apple_transform, apple) in apple_query.iter() {
            if is_colliding(snake_transform, apple_transform, snake.size) {
                commands.entity(apple_entity).despawn();
                spawn_apple_radom_locatint(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut windows,
                );
            }
        }
    }
}

//Precisamos implementar um size para cobra e a maca
fn is_colliding(snake: &Transform, apple: &Transform, snake_size: f32) -> bool {
    let snake_pos = snake.translation;
    let apple_pos = apple.translation;
    let collision_radius = snake_size / 2.0;

    snake_pos.distance(apple_pos) < collision_radius
}
