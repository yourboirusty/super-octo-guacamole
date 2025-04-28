use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::GameState;

#[derive(Resource)]
pub struct LdtkLoading {
    ldtk_handle: LdtkProjectHandle,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        },
    ));

    info!("Loading world");
    let ldtk_handle: LdtkProjectHandle = asset_server.load("world.ldtk").into();
    commands.insert_resource(LdtkLoading {
        ldtk_handle: ldtk_handle.clone(),
    });
    info!("Constructing world");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
    info!("World spawned")
}

pub fn check_asset_loading(
    mut next_state: ResMut<NextState<GameState>>,
    loading_status: Res<LdtkLoading>,
    asset_server: Res<AssetServer>,
) {
    if asset_server.is_loaded_with_dependencies(loading_status.ldtk_handle.clone()) {
        next_state.set(GameState::Playing);
    }
}
