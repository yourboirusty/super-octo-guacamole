use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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
    info!("Constructing world");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
    info!("World spawned")
}
