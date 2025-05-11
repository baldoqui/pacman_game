use bevy::prelude::*;

use crate::plugins::animation::components::AnimationConfig;

#[derive(Component)]
pub struct GameEntity {
    pub position: Vec3,
    pub direction: Vec2,
    pub velocity: f32,
    pub animation: AnimationConfig,
    pub texture: Handle<Image>,
}

impl GameEntity {
    pub fn new(
        position: Vec3,
        direction: Vec2,
        velocity: f32,
        animation: AnimationConfig,
        texture: Handle<Image>,
    ) -> Self {
        GameEntity {
            position,
            direction,
            velocity,
            animation,
            texture,
        }
    }
}

#[derive(Resource, Default)]
pub struct PlayerInputDirection(pub Vec2);

#[derive(Component)]
pub struct PlayerControlled;
