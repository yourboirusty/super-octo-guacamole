use std::time::Duration;

use crate::config::*;
use avian2d::{
    math::{PI, Scalar, Vector},
    prelude::*,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsSchedule, PlayerInputs};

use super::Player;

#[derive(Clone, Copy)]
pub enum InputEnum {
    Right,
    Left,
    Up,
    Down,
    Jump,
    Use,
}

pub struct InputVec(Vec<InputEnum>);

impl From<u8> for InputVec {
    fn from(value: u8) -> Self {
        let mappings = [
            (INPUT_RIGHT, InputEnum::Right),
            (INPUT_LEFT, InputEnum::Left),
            (INPUT_UP, InputEnum::Up),
            (INPUT_DOWN, InputEnum::Down),
            (INPUT_USE, InputEnum::Use),
            (INPUT_FIRE, InputEnum::Jump),
        ];
        let out = mappings
            .iter()
            .filter_map(|(flag, input)| {
                if value & flag != 0 {
                    Some(*input)
                } else {
                    None
                }
            })
            .collect();
        InputVec(out)
    }
}

/// An event sent for a movement input action.
#[derive(Event)]
pub struct MovementEvent {
    entity: Entity,
    action: MovementAction,
}

pub enum MovementAction {
    Move(Scalar),
    Jump,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component, Default)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct Airborne;

#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct Jumping(Duration);

#[derive(Component, Default)]
pub struct CoyoteTime(Duration);

/// The acceleration used for character movement.
#[derive(Component, Reflect, Default)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component, Default, Reflect)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component, Default, Reflect)]
pub struct JumpImpulse(Scalar);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component, Default)]
pub struct MaxSlopeAngle(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle, Default)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(1.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}

pub fn handle_input(
    mut players: Query<(Entity, &Player), With<CharacterController>>,
    inputs: Res<PlayerInputs<MultiplayerConfig>>,
    mut event_writer: EventWriter<MovementEvent>,
) {
    for (entity, player) in &mut players {
        let (raw_input, _) = inputs[player.handle];
        let inputs: InputVec = raw_input.into();

        let mut direction: f32 = 0.;
        for input in inputs.0 {
            match input {
                InputEnum::Right => direction += 1.,
                InputEnum::Left => direction -= 1.,
                InputEnum::Up | InputEnum::Jump => {
                    event_writer.send(MovementEvent {
                        action: MovementAction::Jump,
                        entity,
                    });
                }
                _ => {}
            }
            event_writer.send(MovementEvent {
                action: MovementAction::Move(direction),
                entity,
            });
        }
    }
}
fn update_grounded(
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

fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}

fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementEvent>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Entity,
        Has<Grounded>,
    )>,
) {
    let delta_time = time.delta_secs();

    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, mut linear_velocity, entity, is_grounded) in
            &mut controllers
        {
            if event.entity != entity {
                continue;
            }
            match event.action {
                MovementAction::Move(direction) => {
                    linear_velocity.x += direction * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
                    if is_grounded {
                        linear_velocity.y = jump_impulse.0;
                    }
                }
            }
        }
    }
}

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementEvent>()
            .add_systems(GgrsSchedule, handle_input)
            .add_systems(Update, (update_grounded, apply_movement_damping, movement))
            .register_type::<MovementAcceleration>()
            .register_type::<MovementDampingFactor>();
    }
}
