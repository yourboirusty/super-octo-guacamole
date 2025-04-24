use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: 10.,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}
