use bevy::prelude::*;
use bevy_ggrs::{GgrsSchedule, PlayerInputs, ReadInputs};

use crate::config::*;
use crate::systems::player::Player;

/// Possible movement actions
#[derive(Debug, Clone, Copy)]
pub enum MovementAction {
    /// Move in a direction (handle, direction_vector)
    Move(usize, Vec2),
    /// Jump (handle)
    Jump(usize),
}

/// Event that contains movement information
#[derive(Event)]
pub struct MovementEvent {
    pub action: MovementAction,
}

/// Process inputs and emit movement events
pub fn process_inputs(
    inputs: Res<PlayerInputs<MultiplayerConfig>>,
    mut movement_writer: EventWriter<MovementEvent>,
    players: Query<&Player>,
) {
    // Collect all movement events before sending them
    let mut events = Vec::new();

    for player in &players {
        let (input, _) = inputs[player.handle];

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

        // Add movement event to collection
        events.push(MovementEvent {
            action: MovementAction::Move(player.handle, direction),
        });

        // Check for jump/other actions
        if input & INPUT_FIRE != 0 {
            events.push(MovementEvent {
                action: MovementAction::Jump(player.handle),
            });
        }
    }

    // Send all events at once
    for event in events {
        movement_writer.send(event);
    }
}

/// Plugin that handles controller input and emits movement events
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementEvent>()
            .add_systems(GgrsSchedule, (process_inputs));
    }
}

