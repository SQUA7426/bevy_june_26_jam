use bevy::{
    camera::Viewport,
    prelude::*,
};

fn control_viewport(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    window: Single<&Window>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let (mut _camera, mut transform, mut _projection) = camera_query.into_inner();

    let fspeed = 600.0 * time.delta_secs();
    let _uspeed = fspeed as u32;
    let window_size = window.resolution.physical_size();

    if input.pressed(KeyCode::ArrowUp) {
        if -transform.translation.y < (window_size.y as f32 / 3.8) {
            transform.translation.y -= fspeed;
        }
    }

    if input.pressed(KeyCode::ArrowDown) {
        if transform.translation.y < (window_size.y as f32 / 3.8) {
            transform.translation.y += fspeed;
        }
    }

    if input.pressed(KeyCode::ArrowRight) {
        if -transform.translation.x < (window_size.x as f32 / 3.3) {
            transform.translation.x -= fspeed;
        }
    }

    if input.pressed(KeyCode::ArrowLeft) {
        if transform.translation.x < (window_size.x as f32 / 3.3) {
            transform.translation.x += fspeed;
        }
    }
}

fn setup(
    mut cmds: Commands,
    window: Single<&Window>,
) {
    let window_size = window.resolution.physical_size().as_vec2();

    // DEFAULT CAM
    cmds.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: (window_size * 0.001).as_uvec2(),
                physical_size: (window_size).as_uvec2(),
                ..default()
            }),
            ..default()
        },
    ));

    // TEXT
    cmds.spawn((
        Text::new(
            "Bla, bla, bla...\n\
        USE WASD to move CAM.",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: px(20),
            left: px(10),
            ..default()
        },
    ));
}

#[derive(Debug)]
pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, control_viewport);
    }
}
