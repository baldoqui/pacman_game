use super::components::*;
use bevy::prelude::*;

use crate::plugins::animation::components::AnimationConfig;
use crate::plugins::grid::components::GridCell;

pub fn spawn_game_entity(
    commands: &mut Commands,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    asset_server: &Res<AssetServer>,
    texture_path: &str,
    position: Vec3,
    direction: Vec2,
    velocity: f32,
    animation_config: AnimationConfig,
    sprite_size: UVec2,
    columns: u32,
    rows: u32,
    is_player: bool,
) {
    let texture = asset_server.load(texture_path);
    let game_entity = GameEntity::new(
        position,
        direction,
        velocity,
        animation_config,
        texture.clone(),
    );

    let layout = TextureAtlasLayout::from_grid(sprite_size, columns, rows, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut entity = commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: game_entity.animation.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(1.0)).with_translation(position),
        game_entity.animation.clone(),
        game_entity,
    ));

    if is_player {
        entity.insert(PlayerControlled);
    }
}

pub fn setup_entities(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    spawn_game_entity(
        &mut commands,
        &mut texture_atlas_layouts,
        &asset_server,
        "pacman.png",
        Vec3::ZERO,
        Vec2::new(1.0, 0.0),
        100.0,
        AnimationConfig::new(1, 3, 10),
        UVec2::splat(24),
        4,
        1,
        true,
    );
}

pub fn execute_entities(
    time: Res<Time>,
    mut queryPlayer: Query<(&mut GameEntity, &mut Transform)>,
    mut new_direction: ResMut<PlayerInputDirection>,
) {
    for (mut game_entity, mut transform) in &mut queryPlayer {
        let dir = game_entity.direction;
        let velocity = game_entity.velocity;

        game_entity.position.x += dir.x * velocity * time.delta_secs();
        game_entity.position.y += dir.y * velocity * time.delta_secs();

        let cell_size = 32.0;

        let cell_position = (game_entity.position / cell_size).round() * cell_size;

        if dir.x != 0.0 {
            game_entity.position.y = cell_position.y;
        } else {
            game_entity.position.x = (game_entity.position.x / cell_size).round() * cell_size;
        }

        let tolerance = 1.0;
        let cell_x = (game_entity.position.x / cell_size).round() * cell_size;
        let cell_y = (game_entity.position.y / cell_size).round() * cell_size;

        let is_aligned_x = (game_entity.position.x - cell_x).abs() < tolerance;
        let is_aligned_y = (game_entity.position.y - cell_y).abs() < tolerance;

        if new_direction.0.x != 0.0 && is_aligned_y {
            game_entity.direction = Vec2::new(new_direction.0.x, 0.0);
            game_entity.position.y = cell_y;
        } else if new_direction.0.y != 0.0 && is_aligned_x {
            game_entity.direction = Vec2::new(0.0, new_direction.0.y);
            game_entity.position.x = cell_x;
        }

        transform.translation = game_entity.position;
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut new_direction: ResMut<PlayerInputDirection>,
) {
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        new_direction.0 = Vec2::new(0.0, 1.0);
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        new_direction.0 = Vec2::new(0.0, -1.0);
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        new_direction.0 = Vec2::new(-1.0, 0.0);
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        new_direction.0 = Vec2::new(1.0, 0.0);
    }
}
