use crate::prelude::*;
use avian2d::math::{PI, Scalar};
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::{PlayerInputs, Rollback};
use bevy_tnua::prelude::TnuaController;
use bevy_tnua_avian2d::TnuaAvian2dSensorShape;
use collision_masks::LayerEnum;

use crate::config::*;
use crate::systems::player::Player;

use super::colliders::CharacterCollider;

pub mod collision_masks;

/// Possible movement actions
#[derive(Debug, Clone, Copy)]
pub enum MovementAction {
    /// Move in a direction (handle, direction_vector)
    Move(usize, Vec2),
    /// Jump (handle)
    Jump(usize),
}

/// Event for player actions
#[derive(Event)]
pub struct MovementEvent {
    pub handle: usize,
    pub player: Entity,
    pub actions: Vec<MovementAction>,
}

#[derive(Component, Default, Clone)]
pub struct CharacterController;

#[derive(Component, Default, Clone)]
pub struct Grounded;

#[derive(Clone, Default, Component)]
pub struct MovementAcceleration(pub Scalar);

#[derive(Clone, Default, Component)]
pub struct JumpPower(pub Scalar);

#[derive(Clone, Default, Component)]
pub struct MaxJumpHeight(pub Scalar);

#[derive(Clone, Default, Component)]
pub struct MovementDampingFactor(pub Scalar);

#[derive(Component, Clone)]
pub struct MaxSlopeAngle(pub Scalar);
/// A bundle that contains components for character movement.
///
#[derive(Bundle, Clone)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpPower,
    max_jump_height: MaxJumpHeight,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
        max_jump_height: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpPower(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
            max_jump_height: MaxJumpHeight(max_jump_height),
        }
    }
}
impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45, 100.0)
    }
}

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    collider: Collider,
    body: RigidBody,
    controller: CharacterController,
    collision_mask: CollisionLayers,
    tnua_controller: TnuaController,
    sensor_shape: TnuaAvian2dSensorShape,
    movement: MovementBundle,
    locked_axes: LockedAxes,
}

impl From<CharacterCollider> for CharacterControllerBundle {
    fn from(value: CharacterCollider) -> Self {
        match value {
            CharacterCollider::Player => {
                let collision_mask = CollisionLayers::from(value);
                let collider = Collider::from(value);

                Self::new(collider, collision_mask)
            }
        }
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider, collision_mask: CollisionLayers) -> Self {
        let mut sensor_shape = collider.clone();
        sensor_shape.set_scale(Vec2::new(0.99, 0.99), 1);
        Self {
            controller: CharacterController,
            body: RigidBody::Dynamic,
            collider,
            collision_mask,
            tnua_controller: TnuaController::default(),
            sensor_shape: TnuaAvian2dSensorShape(sensor_shape),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::new(0., 0., 0., 0., 100.),
        }
    }
}

/// Process inputs and emit movement events
pub fn process_inputs(
    inputs: Res<PlayerInputs<MultiplayerConfig>>,
    mut movement_writer: EventWriter<MovementEvent>,
    players: Query<(Entity, &Player), With<Rollback>>,
) {
    for (entity, player) in &players {
        let (input, input_status) = inputs[player.handle];
        match input_status {
            InputStatus::Disconnected => continue,
            _ => {}
        }
        let mut actions = Vec::new();

        let mut direction = Vec2::ZERO;

        if input & INPUT_UP != 0 {
            direction.y += 1.0;
        }
        if input & INPUT_DOWN != 0 {
            direction.y -= 1.0;
        }
        if input & INPUT_RIGHT != 0 {
            direction.x += 1.0;
        }
        if input & INPUT_LEFT != 0 {
            direction.x -= 1.0;
        }

        // Add movement action if direction is non-zero
        actions.push(MovementAction::Move(player.handle, direction));

        // Check for jump/other actions
        if input & INPUT_FIRE != 0 {
            actions.push(MovementAction::Jump(player.handle));
        }

        movement_writer.send(MovementEvent {
            handle: player.handle,
            player: entity,
            actions,
        });
    }
}

/// Plugin that handles controller input and emits movement events
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {}
}
