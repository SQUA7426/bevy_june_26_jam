use crate::{GameState, Player};
use bevy::{
    image::{ImageArrayLayout, ImageLoaderSettings},
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

#[derive(Debug)]
pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(Update, pause_InGame);
    }
}

#[derive(Component, Deref, DerefMut)]
struct UpdateTimer(Timer);

fn setup(mut cmds: Commands, assets: Res<AssetServer>) {
    let chunk_size = UVec2::splat(64);
    let tile_display_size = UVec2::splat(8);
    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
        .map(|i| Some(TileData::from_tileset_index(i as u16)))
        .collect();

    cmds.spawn((
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: assets.load_with_settings(
                "texture/water_texture.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
                },
            ),
            ..default()
        },
        TilemapChunkTileData(tile_data),
        UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Transform::from_translation(Vec3::new(0., 0., -200.)),
    ));
}

fn pause_InGame(mut game_state: ResMut<NextState<GameState>>, input: Res<ButtonInput<KeyCode>>) {
    if input.pressed(KeyCode::Escape) {
        game_state.set(GameState::Pause);
    }
}
