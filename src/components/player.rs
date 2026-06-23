use bevy::{
    color::palettes::basic::PURPLE,
    prelude::{ops::powf, *},
};

use crate::GameState;

#[warn(unused)]
#[derive(Resource)]
struct DecayRate(f32);

#[derive(Resource)]
struct PlayerSpeed {
    value: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Component)]
pub struct Player {
    name: String,
    speed: f32,
    x: f32,
    y: f32,
}

impl Player {
    pub fn new(init_name: String, init_speed: f32, x_pos: f32, y_pos: f32) -> Self {
        Self {
            name: init_name,
            speed: init_speed,
            x: x_pos,
            y: y_pos,
        }
    }
}

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(FixedUpdate, (control_player, update_cam, update_cam_zoom));
    }
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // window: Single<&Window>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // let window_size = window.resolution.physical_size().as_vec2();

    let player = Player::new("Tom".into(), 30., 0., 0.);

    cmds.spawn((
        Mesh2d(meshes.add(Rectangle::new(8., 8.))),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        player.clone(),
    ));

    // TEXT
    cmds.spawn((
        Text::new(
            "Bla, bla, bla...\n\
        USE WASD to move PLAYER.\n\
        USE Period/Comma to ZoomIn/ZoomOut.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: px(20),
            left: px(10),
            ..default()
        },
    ));

    cmds.insert_resource(PlayerSpeed {
        value: player.speed,
    });
    cmds.insert_resource(DecayRate(2.));
}

fn control_player(
    mut player: Single<&mut Transform, With<Player>>,
    speed: Res<PlayerSpeed>,
    input: Res<ButtonInput<KeyCode>>,
    // window: Single<&Window>,
    time: Res<Time<Fixed>>,
) {
    // let window_size = window.size();
    // println!("Window size: x: {} , y: {}", window_size.x, window_size.y);

    let player_speed: f32 = speed.value;

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let movement_delta = direction.normalize_or_zero() * player_speed * time.delta_secs();
    player.translation += movement_delta.extend(0.);
}

fn update_cam(
    mut cam: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    decay_rate: Res<DecayRate>,
    time: Res<Time<Fixed>>,
) {
    let decay_rate = decay_rate.0;

    let delta_time = time.delta_secs();

    let Vec3 { x, y, .. } = player.translation;

    let direction = Vec3::new(x, y, cam.translation.z);

    cam.translation
        .smooth_nudge(&direction, decay_rate, delta_time);
}

fn update_cam_zoom(
    cam_query: Single<(&mut Camera, &mut Projection)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let (mut _cam, mut projection) = cam_query.into_inner();

    if let Projection::Orthographic(projection2d) = &mut *projection {
        if input.pressed(KeyCode::Comma) {
            projection2d.scale *= powf(4.0f32, time.delta_secs());
        }

        if input.pressed(KeyCode::Period) {
            projection2d.scale *= powf(0.25f32, time.delta_secs());
        }
    }
}
