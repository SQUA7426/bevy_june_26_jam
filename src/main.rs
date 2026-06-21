use bevy::prelude::*;

use bevy_june_26_jam::GameStatePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameStatePlugin))
        // .add_plugins(ViewportPlugin)
        .run();
}
