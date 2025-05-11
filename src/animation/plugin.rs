use super::{components::*, systems::*};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, execute_animations).add_systems(
            Update,
            (
                trigger_animation::<RightSprite>.run_if(input_just_pressed(KeyCode::ArrowRight)),
                trigger_animation::<LeftSprite>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
            ),
        );
    }
}
