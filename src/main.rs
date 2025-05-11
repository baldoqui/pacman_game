use bevy::prelude::*;

mod animation;
mod entities;
mod setup;
mod ui;

use animation::plugin::AnimationPlugin;
use entities::plugin::EntitiesPlugin;
use setup::plugin::SetupPlugin;
use ui::plugin::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SetupPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(EntitiesPlugin)
        .run();
}
