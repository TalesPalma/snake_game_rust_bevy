use bevy::prelude::*;

use crate::{apple::apple_systems::*, snake::snake_systems::*};

pub fn behavior_game_system(snake_query: Query<&Snake>, apple_query: Query<&Apple>) {
    //Se a cobra colidir com a maca
    for snake in snake_query.iter() {
        for apple in apple_query.iter() {
            if is_colliding(&snake, &apple) {
                println!("A cobra comeu a maca");
            } else {
                println!("A cobra nao comeu a maca ainda")
            }
        }
    }
}

//Precisamos implementar um size para cobra e a maca
fn is_colliding(snake: &Snake, apple: &Apple) -> bool {
    let position_snake_x = snake.position.x;
    let position_snake_y = snake.position.y;
    let position_apple_x = apple.position.x;
    let position_apple_y = apple.position.y;

    position_apple_y == position_snake_y && position_apple_x == position_snake_x
}
