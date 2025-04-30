use crate::config::*;
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;

use super::Player;

pub fn move_players(
    mut players: Query<(&mut LinearVelocity, &Player)>,
    inputs: Res<PlayerInputs<MultiplayerConfig>>,
    time: Res<Time>,
) {
    for (mut velocity, player) in &mut players {
        let (input, _) = inputs[player.handle];

        let mut direction = Vec2::ZERO;

        if input & INPUT_UP != 0 {
            direction.y += 10.;
        }
        if input & INPUT_DOWN != 0 {
            direction.y -= 10.;
        }
        if input & INPUT_RIGHT != 0 {
            direction.x += 1.;
        }
        if input & INPUT_LEFT != 0 {
            direction.x -= 1.;
        }
        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 20.;
        let acceleration = 100.;
        direction = direction.normalize();
        velocity.0 += direction * acceleration * time.delta_secs();
        if direction.x > 0. && velocity.0.x > move_speed {
            velocity.0.x = move_speed;
        } else if direction.x < 0. && velocity.0.x < -move_speed {
            velocity.0.x = -move_speed;
        }
    }
}
