use bevy::prelude::*;

mod animation;
mod setup;
mod ui;

use setup::plugin::SetupPlugin;
use ui::plugin::UiPlugin;
use animation::plugin::AnimationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SetupPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(AnimationPlugin)
        .run();
}
