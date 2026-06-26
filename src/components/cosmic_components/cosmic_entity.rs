use crate::{GameState, Player, components::cosmic_components::cosmic_resource::CeCreationTimer};
use bevy::prelude::*;
use rand::prelude::*;

use crate::CosmicEssence;
use crate::components::cosmic_components::cosmic_type::CosmicType;

// ENTITY
#[allow(dead_code)]
#[derive(Component, Clone, Debug)]
pub struct CosmicEntity {
    pub pos: Vec2,
    pub radius: f32,
    pub cosmic_essences: Vec<CosmicEssence>,
    pub production_rate: f32,
    pub entity_type: CosmicType,
}

fn produce_rate(radius: f32) -> f32 {
    let lower = radius * 0.01;
    let upper = lower + radius * 0.025;
    rand::rng().random_range(lower..upper)
}

#[derive(Component, Debug)]
struct CosmicEssenceText;

#[derive(Debug)]
pub struct CosmicPlugin;

impl Plugin for CosmicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), cosmic_setup)
            .add_systems(
                Update,
                (update_cosmic_essence, calc_circle, cosmic_production).chain(),
            );
    }
}

fn cosmic_setup(mut cmds: Commands) {
    cmds.insert_resource(CosmicEssence::new(20.));
    cmds.insert_resource(CeCreationTimer(Timer::from_seconds(
        10.,
        TimerMode::Repeating,
    )));
    cmds.spawn((
        Text::new("Cosmic Essence: 20.0"),
        Node {
            position_type: PositionType::Absolute,
            top: px(20),
            left: percent(50.),
            ..default()
        },
        CosmicEssenceText,
    ));
}

fn update_cosmic_essence(
    essence: Option<Res<CosmicEssence>>,
    mut query_text: Query<&mut Text, With<CosmicEssenceText>>,
) {
    let Some(essence) = essence else { return };

    if !essence.is_changed() {
        return;
    }

    for mut text in &mut query_text {
        text.0 = format!("Cosmic Essence: {:.1}", essence.value);
    }
}

fn cosmic_production(
    query: Option<Query<&CosmicEntity>>,
    essence: Option<ResMut<CosmicEssence>>,
    time: Res<Time>,
) {
    let Some(query) = query else { return };
    let Some(mut essence) = essence else { return };

    if query.is_empty() {
        return;
    }
    for entity in &query {
        essence.value += entity.production_rate * time.delta_secs();
    }
}

fn calc_circle(
    mut cmds: Commands,
    player_query: Single<&mut Player>,
    essences: Option<Res<CosmicEssence>>,
    timer: Option<Res<CeCreationTimer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Some(essences) = essences else { return };
    let Some(timer) = timer else { return };

    let ce = essences.value;

    if ce < 20. {
        if timer.0.just_finished() {
            println!("20.0 CosmicEssences are required to form a NeuronStar!");
        }
        return;
    }

    let mut player = player_query.into_inner();

    let len = player.pos.len();

    if len < 50 {
        return;
    }

    let centroid = player.pos.iter().fold(Vec2::ZERO, |pre, pos| pre + pos) / len as f32;

    let avg_radius = player
        .pos
        .iter()
        .map(|point| point.distance(centroid))
        .sum::<f32>()
        / len as f32;

    let variance = player
        .pos
        .iter()
        .map(|point| (point.distance(centroid) - avg_radius).powi(2))
        .sum::<f32>()
        / len as f32;

    let std_dev = variance.sqrt();

    if std_dev >= avg_radius * 0.4 || avg_radius <= 20. {
        return;
    }

    let mut angles: Vec<f32> = player
        .pos
        .iter()
        .map(|p| (p.y - centroid.y).atan2(p.x - centroid.x))
        .collect();
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let max_gap = angles
        .windows(2)
        .map(|w| w[1] - w[0])
        .chain(std::iter::once(
            angles.first().unwrap() + std::f32::consts::TAU - angles.last().unwrap(),
        ))
        .fold(0.0f32, f32::max);

    if max_gap < std::f32::consts::FRAC_PI_2 {
        player.pos.clear();

        cmds.spawn((
            CosmicEntity {
                pos: centroid,
                radius: avg_radius,
                cosmic_essences: Vec::new(),
                production_rate: produce_rate(avg_radius),
                entity_type: CosmicType::NeuronStar,
            },
            Mesh2d(meshes.add(Circle::new(avg_radius))),
            MeshMaterial2d(materials.add(Color::linear_rgb(50., 50., 0.))),
            Transform::from_translation(Vec3::new(centroid.x, centroid.y, 10.)),
        ));
        cmds.insert_resource(CosmicEssence { value: ce - 20. });
        println!(
            "CosmicEntity created — pos: {}, radius: {:.1}, rate: {:.2}",
            centroid,
            avg_radius,
            produce_rate(avg_radius)
        );
    }
}
