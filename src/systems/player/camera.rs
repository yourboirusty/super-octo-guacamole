use bevy::{prelude::*, render::camera};
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::LocalPlayers;

use crate::systems::multiplayer::Local;

use super::Player;

const ASPECT_RATIO: f32 = 4. / 3.;
const PLAYER_EDGE: f32 = -10.;

// Stores global coordinates for bottom left and top right corners of a camera view
#[derive(Component, Default)]
struct CameraView(Vec2, Vec2);

pub fn camera_follow_local_players(
    player_locations_q: Query<(&GlobalTransform), (With<Player>, With<Local>)>,
    mut camera_q: Query<(&mut Transform), (With<Camera2d>, Without<Player>)>,
) {
    let Ok(mut camera_transform) = camera_q.get_single_mut() else {
        return;
    };
    let Ok(player) = player_locations_q.get_single() else {
        return;
    };
    camera_transform.translation = player.translation();
}
