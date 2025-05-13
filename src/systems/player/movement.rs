use crate::systems::controller::{
    CharacterController, Grounded, JumpPower, MaxSlopeAngle, MovementAcceleration, MovementAction,
    MovementDampingFactor, MovementEvent,
};
use avian2d::{
    math::{Scalar, Vector},
    prelude::*,
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

        //// For some reason, if you wanted to zero out a velocity and it happens
        //// that the two players are in contact, it will cause a desync in Avian
        //let new_vel_x = if horizontal != 0. {
        //    horizontal
        //} else {
        //    velocity.x
        //};
        //
        //let new_vel_y = if vertical != 0. { vertical } else { velocity.y };

        velocity.x = horizontal;
        velocity.y = vertical;
    }
}

/// Slows down movement in the X direction.
pub fn apply_movement_damping(
    mut query: Query<(&MovementDampingFactor, &mut LinearVelocity), With<Rollback>>,
) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}

pub fn apply_gravity(
    mut query: Query<
        &mut LinearVelocity,
        (With<CharacterController>, Without<Grounded>, With<Rollback>),
    >,
    time: Res<Time<Physics>>,
    gravity: Res<Gravity>,
) {
    if time.is_paused() {
        return;
    }
    for mut velocity in &mut query {
        velocity.y += gravity.0.y * time.delta_secs();
        velocity.x += gravity.0.x * time.delta_secs();
    }
}

pub fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        (With<CharacterController>, With<Rollback>),
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

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system handles collision response for kinematic character controllers
/// by pushing them along their contact normals by the current penetration depth,
/// and applying velocity corrections in order to snap to slopes, slide along walls,
/// and predict collisions using speculative contacts.
#[allow(clippy::type_complexity)]
pub fn kinematic_controller_collisions(
    collisions: Res<Collisions>,
    bodies: Query<&RigidBody>,
    collider_parents: Query<&ColliderParent, Without<Sensor>>,
    mut character_controllers: Query<
        (
            &mut Position,
            &Rotation,
            &mut LinearVelocity,
            Option<&MaxSlopeAngle>,
        ),
        (With<RigidBody>, With<CharacterController>, With<Rollback>),
    >,
    time: Res<Time<Physics>>,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([collider_parent1, collider_parent2]) =
            collider_parents.get_many([contacts.entity1, contacts.entity2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;

        let character_rb: RigidBody;
        let is_other_dynamic: bool;

        let (mut position, rotation, mut linear_velocity, max_slope_angle) =
            if let Ok(character) = character_controllers.get_mut(collider_parent1.get()) {
                is_first = true;
                character_rb = *bodies.get(collider_parent1.get()).unwrap();
                is_other_dynamic = bodies
                    .get(collider_parent2.get())
                    .is_ok_and(|rb| rb.is_dynamic());
                character
            } else if let Ok(character) = character_controllers.get_mut(collider_parent2.get()) {
                is_first = false;
                character_rb = *bodies.get(collider_parent2.get()).unwrap();
                is_other_dynamic = bodies
                    .get(collider_parent1.get())
                    .is_ok_and(|rb| rb.is_dynamic());
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers.
        if !character_rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.global_normal1(rotation)
            } else {
                -manifold.global_normal2(rotation)
            };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.contacts.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            // Determine if the slope is climbable or if it's too steep to walk on.
            let slope_angle = normal.angle_to(Vector::Y);
            let climbable = max_slope_angle.is_some_and(|angle| slope_angle.abs() <= angle.0);

            if deepest_penetration > 0.0 {
                // If the slope is climbable, snap the velocity so that the character
                // up and down the surface smoothly.
                if climbable {
                    // Points either left or right depending on which side the normal is leaning on.
                    // (This could be simplified for 2D, but this approach is dimension-agnostic)
                    let normal_direction_x =
                        normal.reject_from_normalized(Vector::Y).normalize_or_zero();

                    // The movement speed along the direction above.
                    let linear_velocity_x = linear_velocity.dot(normal_direction_x);

                    // Snap the Y speed based on the speed at which the character is moving
                    // up or down the slope, and how steep the slope is.
                    //
                    // A 2D visualization of the slope, the contact normal, and the velocity components:
                    //
                    //             ╱
                    //     normal ╱
                    // *         ╱
                    // │   *    ╱   velocity_x
                    // │       * - - - - - -
                    // │           *       | velocity_y
                    // │               *   |
                    // *───────────────────*

                    let max_y_speed = -linear_velocity_x * slope_angle.tan();
                    linear_velocity.y = linear_velocity.y.max(max_y_speed);
                } else {
                    // The character is intersecting an unclimbable object, like a wall.
                    // We want the character to slide along the surface, similarly to
                    // a collide-and-slide algorithm.

                    // Don't apply an impulse if the character is moving away from the surface.
                    if linear_velocity.dot(normal) > 0.0 {
                        continue;
                    }

                    // Slide along the surface, rejecting the velocity along the contact normal.
                    let impulse = linear_velocity.reject_from_normalized(normal);
                    linear_velocity.0 = impulse;
                }
            } else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = linear_velocity.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude = normal_speed - (deepest_penetration / time.delta_secs());
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.
                if climbable {
                    // Avoid sliding down slopes.
                    linear_velocity.y -= impulse.y.min(0.0);
                } else {
                    // Avoid climbing up walls.
                    impulse.y = impulse.y.max(0.0);
                    linear_velocity.0 -= impulse;
                }
            }
        }
    }
}
