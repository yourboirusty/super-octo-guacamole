use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

use crate::components::Player;

pub fn spawn(mut commands: Commands) {
    commands
        .spawn((
            Player::new(0),
            Sprite {
                color: Color::srgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
        ))
        .add_rollback();

    // Player 2
    commands
        .spawn((
            Player::new(1),
            Transform::from_translation(Vec3::new(2., 0., 0.)),
            Sprite {
                color: Color::srgb(0., 0.4, 0.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
        ))
        .add_rollback();
}
