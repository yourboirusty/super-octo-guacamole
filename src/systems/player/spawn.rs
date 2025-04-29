use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::ldtk_pixel_coords_to_translation_pivoted;

use crate::config::PLAYER_Z;

use super::Player;

#[derive(Default, Bundle)]
pub struct SpawnPointBundle {
    locations: SpawnLocation,
}
#[derive(Default, Component, Clone)]
pub struct SpawnLocation(Vec2);

#[derive(Default, Clone)]
pub struct PlayerSpawnState {
    pub loaded_spawns: Vec<Vec2>,
    original_spawns: Vec<Vec2>,
    pub players_waiting: Vec<Entity>,
}

impl PlayerSpawnState {
    pub fn get_spawn(&mut self) -> Vec2 {
        let result = match self.loaded_spawns.pop() {
            None => {
                self.loaded_spawns = self.original_spawns.clone();
                None
            }
            Some(spawn) => Some(spawn),
        };
        if result.is_none() && self.original_spawns.len() == 0 {
            panic!("Trying to get spawns that aren't loaded");
        }
        result.expect("")
    }
}

#[derive(Event)]
pub struct SpawnPlayerEvent(pub Entity);

impl LdtkEntity for SpawnPointBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlasLayout>,
    ) -> SpawnPointBundle {
        let spawn = ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        );

        let locations = SpawnLocation(spawn);

        SpawnPointBundle { locations }
    }
}
pub fn spawn_player(
    mut spawn_points_q: Query<(Entity, &mut SpawnLocation)>,
    mut players_q: Query<&mut Transform, With<Player>>,
    mut player_entered: EventReader<SpawnPlayerEvent>,
    level_query: Query<(Entity, &LevelIid)>,
    level_selection: Res<LevelSelection>,
    mut local: Local<PlayerSpawnState>,
    mut commands: Commands,
) {
    for (entity, spawn_location) in &mut spawn_points_q {
        local.loaded_spawns.push(spawn_location.0.clone());
        commands.entity(entity).despawn();
    }

    if local.loaded_spawns.len() == 0 {
        return;
    }

    for player_event in &mut player_entered.read() {
        local.players_waiting.push(player_event.0);
    }

    let current_level_iid = match &*level_selection {
        LevelSelection::Iid(iid) => iid,
        _ => panic!("Please use iid only thx uwu"),
    };

    let mut parent_entity_option: Option<Entity> = None;

    for (entity, level_id) in &level_query {
        if *level_id == *current_level_iid {
            parent_entity_option = Some(entity.clone());
            break;
        }
    }

    let current_level_entity = parent_entity_option.expect("Current level not found");

    while let Some(player) = local.players_waiting.pop() {
        if local.loaded_spawns.len() == 0 {
            local.players_waiting.push(player);
            return;
        }
        info!("Spawning player {}", player);
        let mut player_transform = players_q
            .get_mut(player)
            .expect("Event sent about nonexisting player");
        let spawn = local.get_spawn();
        player_transform.translation = Vec3::from_array([spawn.x, spawn.y, PLAYER_Z]);
        commands.entity(player).set_parent(current_level_entity);
    }
}
