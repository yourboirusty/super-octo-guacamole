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
    // Pre-process events to get the most recent move event for each player
    let mut latest_moves = std::collections::HashMap::new();
    let mut jumps = Vec::new();
    
    for event in movement_events.read() {
        match event.action {
            MovementAction::Move(handle, dir) => {
                latest_moves.insert(handle, dir);
            }
            MovementAction::Jump(handle) => {
                jumps.push(handle);
            }
        }
    }
    
    // Process moves (we only care about the latest movement per player)
    for (handle, dir) in latest_moves {
        // Skip if direction is zero
        if dir == Vec2::ZERO {
            continue;
        }
        
        for (mut velocity, player) in &mut players {
            if player.handle == handle {
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
    }
    
    // Process jumps
    for handle in jumps {
        // Jump logic could be implemented here 
    }
}
