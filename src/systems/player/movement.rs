use crate::systems::controller::{
    CharacterController, Grounded, JumpPower, MaxSlopeAngle, MovementAcceleration, MovementAction,
    MovementDampingFactor, MovementEvent,
};
use avian2d::{
    math::Vector,
    prelude::{LinearVelocity, Physics, PhysicsTime, Rotation, ShapeHits},
};
use bevy::prelude::*;
use bevy_ggrs::Rollback;

use super::Player;

pub fn move_players(
    mut players: Query<
        (
            &mut LinearVelocity,
            &Player,
            &MovementAcceleration,
            &JumpPower,
            Has<Grounded>,
        ),
        With<Rollback>,
    >,
    mut movement_events: EventReader<MovementEvent>,
    time: Res<Time<Physics>>,
) {
    let mut latest_moves = std::collections::HashMap::new();
    let mut jumps = Vec::new();

    // Collect the latest moves and jumps in a single pass
    for event in movement_events.read() {
        for action in &event.actions {
            match action {
                MovementAction::Move(handle, dir) => {
                    latest_moves.insert(*handle, *dir);
                }
                MovementAction::Jump(handle) => {
                    jumps.push(*handle);
                }
            }
        }
    }

    if time.is_paused() {
        return;
    }

    // Process both moves and jumps in a single player loop
    for (mut velocity, player, acceleration, jump_power, is_grounded) in &mut players {
        let mut horizontal = velocity.x;
        let mut vertical = velocity.y;

        // Process movement if this player has a move action
        if let Some(dir) = latest_moves.get(&player.handle) {
            if *dir != Vec2::ZERO {
                let move_speed = 20.;

                let direction = dir.normalize();
                horizontal += direction.x * acceleration.0 * 10. * time.delta_secs();

                if direction.x > 0. && velocity.0.x > move_speed {
                    horizontal = move_speed;
                } else if direction.x < 0. && velocity.0.x < -move_speed {
                    horizontal = -move_speed;
                }
            }
        }

        if jumps.contains(&player.handle) {
            if is_grounded {
                vertical = jump_power.0 * 60. * time.delta_secs();
            }
        }

        // For some reason, if you wanted to zero out a velocity and it happens
        // that the two players are in contact, it will cause a desync in Avian
        let new_vel_x = if horizontal != 0. {
            horizontal
        } else {
            velocity.x
        };

        let new_vel_y = if vertical != 0. { vertical } else { velocity.y };

        velocity.x = new_vel_x;
        velocity.y = new_vel_y;
    }
}

/// Slows down movement in the X direction.
pub fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}

pub fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_to(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}
