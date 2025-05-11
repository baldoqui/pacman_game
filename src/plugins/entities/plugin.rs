use super::systems::*;
use bevy::prelude::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_entities)
            .add_systems(Update, (execute_entities, player_movement));
    }
}
