use crate::config::*;
use crate::systems::controller::{MovementAction, MovementEvent};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use super::Player;

pub fn move_players(
    mut players: Query<(&mut LinearVelocity, &Player)>,
    mut movement_events: EventReader<MovementEvent>,
    time: Res<Time>,
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
    
    // Process both moves and jumps in a single player loop
    for (mut velocity, player) in &mut players {
        // Process movement if this player has a move action
        if let Some(dir) = latest_moves.get(&player.handle) {
            if *dir != Vec2::ZERO {
                let move_speed = 20.;
                let acceleration = 100.;
                
                let direction = dir.normalize();
                velocity.0 += direction * acceleration * time.delta_secs();
                
                if direction.x > 0. && velocity.0.x > move_speed {
                    velocity.0.x = move_speed;
                } else if direction.x < 0. && velocity.0.x < -move_speed {
                    velocity.0.x = -move_speed;
                }
            }
        }
        
        // Process jump if this player has a jump action
        if jumps.contains(&player.handle) {
            // Jump logic could be implemented here
        }
    }
}
