use bevy::prelude::*;

mod animation;
mod setup;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .run();
}
