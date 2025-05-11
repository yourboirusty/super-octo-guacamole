mod camera;
pub mod movement;
pub mod spawn;

use avian2d::prelude::SleepingDisabled;
pub use spawn::*;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::GameState;

use super::controller::CharacterControllerBundle;

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,

    pub character_controller: CharacterControllerBundle,
    pub no_sleep: SleepingDisabled,

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
