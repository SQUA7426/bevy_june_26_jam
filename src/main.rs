use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};

use bevy_june_26_jam::{
    CosmicPlugin, DebugTextPlugin, GameStatePlugin, MousePlugin, PlayerPlugin, TilemapPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_linear())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_plugins((
            GameStatePlugin,
            MousePlugin,
            PlayerPlugin,
            TilemapPlugin,
            DebugTextPlugin,
            CosmicPlugin,
        ))
        .run();
}
