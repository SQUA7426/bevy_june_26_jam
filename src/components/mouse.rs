use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::cosmic_components::cosmic_entity::CosmicEntity;

#[derive(Debug)]
pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (select).chain());
    }
}

fn get_cursor_pos(
    window: &Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Option<Vec2> {
    let (cam, g_transform) = camera.into_inner();

    let cursor_pos = window.cursor_position()?;

    cam.viewport_to_world_2d(g_transform, cursor_pos).ok()
}

fn mouse_in_cosmic_entity(mouse_pos: Vec2, ce: &CosmicEntity) -> bool {
    let center_x = ce.pos.x;
    let center_y = ce.pos.y;
    (mouse_pos.x - center_x).powi(2) + (mouse_pos.y - center_y).powi(2) < ce.radius.powi(2)
}

fn select(
    mut cmds: Commands,
    cam: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    query_mesh_ce: Option<Query<(&MeshMaterial2d<ColorMaterial>, &CosmicEntity), With<CosmicEntity>>>,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Some(query_mesh_ce) = query_mesh_ce else { return };
    // FOR COMBINE OR SMTH
    if mouse_btn.just_pressed(MouseButton::Left) {
        // println!("Left Mouse Button pressed!");

        let Some(pos) = get_cursor_pos(&window, cam) else {
            return;
        };
        for query in &query_mesh_ce {
            let (meshes, ce) = query.into();
            // println!("CosmicEntity at pos: {}, radius: {}, mouse_pos: {}", ce.pos, ce.radius, pos);
            if mouse_in_cosmic_entity(pos, ce) {
                cmds.insert_resource(CosmicEntityClicked { is_clicked: true });
                // println!("Mouse on CosmicEntity!");
                if let Some(material) = materials.get_mut(&meshes.0) {
                    material.color = if material.color == Color::linear_rgb(50., 50., 0.) {
                        Color::linear_rgb(0., 50., 50.)
                    } else {
                        Color::linear_rgb(50., 50., 0.)
                    };
                    cmds.insert_resource(CosmicEntityClicked { is_clicked: false });
                }
            }
        }
    }

    // INFORMATION
    if mouse_btn.just_pressed(MouseButton::Right) {
        // println!("Right Mouse Button pressed!");
    }
}
