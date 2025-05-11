use super::components::*;
use bevy::prelude::*;

use crate::animation::components::AnimationConfig;

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

pub fn execute_entities(time: Res<Time>, mut query: Query<(&mut GameEntity, &mut Transform)>) {
    for (mut game_entity, mut transform) in &mut query {
        let dir = game_entity.direction;
        let velocity = game_entity.velocity;

        game_entity.position.x += dir.x * velocity * time.delta_secs();
        game_entity.position.y += dir.y * velocity * time.delta_secs();

        transform.translation = game_entity.position;
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut GameEntity, With<PlayerControlled>>,
) {
    for mut game_entity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            game_entity.direction.y = 1.0;
            game_entity.direction.x = 0.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            game_entity.direction.y = -1.0;
            game_entity.direction.x = 0.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            game_entity.direction.y = 0.0;
            game_entity.direction.x = -1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            game_entity.direction.y = 0.0;
            game_entity.direction.x = 1.0;
        }
    }
}
