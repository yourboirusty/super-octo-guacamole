use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: 10.,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("test.ldtk").into(),
        ..Default::default()
    });
}
