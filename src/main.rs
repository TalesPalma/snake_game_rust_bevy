use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
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
