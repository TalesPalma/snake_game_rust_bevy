use bevy::prelude::*;

pub struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Component)]
pub struct Snake {
    pub(crate) segments: Vec<Position>,
    pub(crate) direction: DirectionMoviment,
}

pub(crate) enum DirectionMoviment {
    Up,
    Down,
    Left,
    Right,
}

pub fn move_snake_controller_system(mut query: Query<&mut Snake>) {
    for mut snake in query.iter_mut() {
        match snake.direction {
            DirectionMoviment::Up => snake.segments[0].y += 2.0,
            DirectionMoviment::Down => snake.segments[0].y -= 2.0,
            DirectionMoviment::Left => snake.segments[0].x -= 2.0,
            DirectionMoviment::Right => snake.segments[0].x += 2.0,
        }
    }
}

pub fn move_snake_in_screen(mut query: Query<(&mut Snake, &mut Transform)>) {
    for (mut snake, mut transform) in query.iter_mut() {
        transform.translation.x = snake.segments[0].x;
        transform.translation.y = snake.segments[0].y;
    }
}

pub fn keyboard_input(mut query: Query<&mut Snake>, keyborad: Res<ButtonInput<KeyCode>>) {
    for mut snake in query.iter_mut() {
        if keyborad.pressed(KeyCode::KeyW) {
            snake.direction = DirectionMoviment::Up;
        }
        if keyborad.pressed(KeyCode::KeyS) {
            snake.direction = DirectionMoviment::Down;
        }
        if keyborad.pressed(KeyCode::KeyA) {
            snake.direction = DirectionMoviment::Left;
        }
        if keyborad.pressed(KeyCode::KeyD) {
            snake.direction = DirectionMoviment::Right;
        }
    }
}
