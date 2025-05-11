use crate::animation::components::{AnimationConfig, LeftSprite, RightSprite};
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

  let texture = asset_server.load("pacman.png");

  let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 4, 1, None, None);
  let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_config_1 = AnimationConfig::new(1, 3, 10);
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_1.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(-70.0, 0.0, 0.0)),
        LeftSprite,
        animation_config_1,
    ));

    let animation_config_2 = AnimationConfig::new(1, 3, 20);
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config_2.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(70.0, 0.0, 0.0)),
        RightSprite,
        animation_config_2,
    ));
}
