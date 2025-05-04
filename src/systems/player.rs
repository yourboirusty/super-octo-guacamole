mod camera;
mod ground_detection;
mod movement;
mod spawn;

use bevy_ggrs::GgrsSchedule;
pub use camera::*;
pub use movement::*;
pub use spawn::*;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use spawn::spawn_player;

use crate::game::GameState;

pub enum MovementState {
    Grounded,
    Airborne,
    Coyote(f32),
}

#[derive(Default, Component, PartialEq, Eq, Clone, Copy)]
pub struct Player {
    pub handle: usize,
}

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,

    pub character_controller: CharacterControllerBundle,

    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    pub transform: Transform,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<SpawnPointBundle>("SpawnPoint")
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_systems(Last, spawn_player.run_if(in_state(GameState::Playing)));
    }
}
