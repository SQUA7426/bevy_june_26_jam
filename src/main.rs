use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};

use bevy_june_26_jam::{DebugTextPlugin, GameStatePlugin, PlayerPlugin, TilemapPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
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
            PlayerPlugin,
            TilemapPlugin,
            DebugTextPlugin,
        ))
        .run();
}
