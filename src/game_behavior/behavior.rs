use bevy::prelude::*;

use crate::{
    apple::apple_systems::*,
    snake::{self, snake_systems::*},
};

pub fn behavior_game_system(snake_query: Query<&Snake>, apple_query: Query<&Apple>) {
    //Se a cobra colidir com a maca
    for snake in snake_query.iter() {
        for apple in apple_query.iter() {
            if snake.position == apple.position {
                println!("A cobra comeu a maca");
            }
        }
    }
    //Chame a funcao de checar colisao
    //E depois de checar a colisao , remova aquela maca e add um nova maca
    //E adicione a maca em um position aleatoria no caso chamando a funcao de spawn radom
    //locating
}
