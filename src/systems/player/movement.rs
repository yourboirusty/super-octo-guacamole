use crate::{components::Player, config::*};
use bevy::prelude::*;
use bevy_ggrs::PlayerInputs;

pub fn move_players(
    mut players: Query<(&mut Transform, &Player)>,
    inputs: Res<PlayerInputs<MultiplayerConfig>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        let (input, _) = inputs[player.handle];

        let mut direction = Vec2::ZERO;

        if input & INPUT_UP != 0 {
            direction.y += 1.;
        }
        if input & INPUT_DOWN != 0 {
            direction.y -= 1.;
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

        let move_speed = 7.;
        let move_delta = direction * move_speed * time.delta_secs();
        transform.translation += move_delta.extend(0.);
    }
}
