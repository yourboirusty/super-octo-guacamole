mod camera;
mod ground_detection;
mod movement;
pub mod spawn;

pub use spawn::*;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::GameState;

use super::colliders::ColliderBundle;

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,

    pub collider_bundle: ColliderBundle,

    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    pub transform: Transform,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement::move_players)
            .add_event::<SpawnPlayerEvent>()
            .register_ldtk_entity::<SpawnPointBundle>("SpawnPoint")
            .add_systems(Last, spawn_player.run_if(in_state(GameState::Playing)))
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}
