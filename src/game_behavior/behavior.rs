use bevy::prelude::*;

use crate::{
    apple::apple_systems::{self, spawn_apple_radom_locatint, Apple},
    snake::snake_systems::Snake,
};

pub fn behavior_game_system(mut commands: Commands, mut entitys: Query<(&mut Snake, &mut Apple)>) {
    for (mut snake, mut apple) in entitys.iter_mut() {
        //Se a cobra colidir com a maca
        //Chame a funcao de checar colisao
        //E depois de checar a colisao , remova aquela maca e add um nova maca
        //E adicione a maca em um position aleatoria no caso chamando a funcao de spawn radom
        //locating
    }
}
