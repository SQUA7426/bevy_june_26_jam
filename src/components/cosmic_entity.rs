use crate::components::player::{Player};
use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
struct CosmicEntity {
    pub pos: Vec2,
    pub radius: f32,
}

#[derive(Debug)]
pub struct CosmicPlugin;

#[allow(unused)]
#[derive(Component)]
struct CircleMaker(CosmicEntity);

impl Plugin for CosmicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (calc_circle, draw_cirlce));
    }
}

fn calc_circle(mut cmds: Commands, player_query: Single<(&mut Player, &mut Transform)>) {
    let (mut player, _transform) = player_query.into_inner();

    let len = player.pos.len();

    if len < 25 {
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

    let deviation = 0.4;
    let allowed = avg_radius * deviation;

    let std_dev = variance.sqrt();

    if std_dev >= allowed || avg_radius <= 8. {
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

    let max_allowed_gap = std::f32::consts::FRAC_PI_2;

    if max_gap < max_allowed_gap {
        cmds.insert_resource(CosmicEntity {
            pos: centroid,
            radius: avg_radius,
        });
        player.pos.clear();
        println!("CosmicEntity was created!");
    }
}

fn draw_cirlce(
    mut cmds: Commands,
    ce: Option<Res<CosmicEntity>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Some(ce) = ce else { return };

    let v3 = Vec3::new(ce.pos.x, ce.pos.y, 10.);

    cmds.spawn((
        Mesh2d(meshes.add(Circle::new(ce.radius))),
        MeshMaterial2d(materials.add(Color::linear_rgb(50., 50., 0.))),
        Transform::from_translation(v3),
        CircleMaker(CosmicEntity {
            pos: ce.pos,
            radius: ce.radius,
        }),
    ));
}
