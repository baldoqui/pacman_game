use bevy::prelude::*;

mod plugins;

use plugins::{
    animation::plugin::AnimationPlugin, entities::plugin::EntitiesPlugin, grid::plugin::GridPlugin,
    setup::plugin::SetupPlugin, ui::plugin::UiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SetupPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(EntitiesPlugin)
        .add_plugins(GridPlugin)
        .run();
}
