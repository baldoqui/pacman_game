use super::components::*;
use bevy::prelude::*;

pub fn spawn_grid(mut commands: Commands) {
    let grid_width = 10;
    let grid_height = 10;
    let cell_size = 32.0;

    let color_blue = Color::linear_rgb(0.0, 0.0, 1.0);

    for y in 0..grid_height {
        for x in 0..grid_width {
            commands.spawn((
                Sprite {
                    color: color_blue,
                    custom_size: Some(Vec2::splat(cell_size - 2.0)),
                    ..default()
                },
                Transform::from_xyz(x as f32 * cell_size, y as f32 * cell_size, 0.0),
                GridCell { x, y },
            ));
        }
    } 
}

pub fn update_grid(mut query: Query<(&GridCell, &mut Sprite)>) {
    let color_blue = Color::linear_rgb(0.0, 0.0, 1.0);

    for (cell, mut sprite) in query.iter_mut() {
        if cell.x == cell.y {
            sprite.color = color_blue;
        }
    }
}
