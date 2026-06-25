use std::{collections::VecDeque, time::Duration};

use bevy::{prelude::*, ui::widget::TextUiWriter};

use crate::{GameState, Player};

#[derive(Debug)]
pub struct DebugTextPlugin;

impl Plugin for DebugTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(Update, change_text_system);
    }
}

#[derive(Component, Resource)]
struct TextChanges;

fn setup(mut cmds: Commands) {
    cmds.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(20),
            left: px(10),
            ..default()
        },
        TextChanges,
    ));
}

fn change_text_system(
    mut fps_history: Local<VecDeque<f64>>,
    mut time_history: Local<VecDeque<Duration>>,
    player: Single<&Player>,
    time: Res<Time>,
    query: Query<Entity, With<TextChanges>>,
    mut writer: TextUiWriter,
) {
    time_history.push_front(time.elapsed());
    time_history.truncate(120);

    let avg_fps = (time_history.len() as f64)
        / (time_history.front().copied().unwrap_or_default()
            - time_history.back().copied().unwrap_or_default())
        .as_secs_f64()
        .max(0.0001);

    fps_history.push_front(avg_fps);
    fps_history.truncate(120);

    let binding = player.clone();
    let pos = match binding.pos.last() {
        Some(v) => { (v.x, v.y) },
        None => { (0.,0.) }
    };

    for entity in &query {
        *writer.text(entity, 0) = format!("{:.1} avg fps\nPlayer pos: x: {:.1}, y: {:.?}", avg_fps, pos.0, pos.1);
    }
}
