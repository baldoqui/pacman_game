use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Left Arrow: Animate Left Sprite\nRight Arrow: Animate Right Sprite"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}
