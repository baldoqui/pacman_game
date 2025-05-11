use super::components::*;
use bevy::prelude::*;

const WALL_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.5);
const EMPTY_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.0);
const DOT_COLOR: Color = Color::linear_rgb(1.0, 1.0, 0.0);
const POWERUP_COLOR: Color = Color::linear_rgb(1.0, 0.0, 1.0);

pub fn spawn_grid(mut commands: Commands) {
    let cell_size = 32.0;
    let game_map = GameMap::classic();

    let grid_width = game_map.width;
    let grid_height = game_map.height;

    for y in 0..grid_height {
        for x in 0..grid_width {
            let cell = game_map.get(x, y).unwrap();

            let color = match cell.cell_type {
                CellType::Wall => WALL_COLOR,
                CellType::Empty => EMPTY_COLOR,
                CellType::Dot => DOT_COLOR,
                CellType::PowerUp => POWERUP_COLOR,
            };

            let position = Vec2::new(x as f32 * cell_size, (y as f32 * cell_size));

            let position = position - 10.0 * cell_size;

            commands.spawn((
                Sprite {
                    color: color,
                    custom_size: Some(Vec2::splat(cell_size - 2.0)),
                    ..default()
                },
                Transform::from_xyz(position.x, position.y, 0.0),
            ));
        }
    }
}

