use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use config::MultiplayerConfig;

mod components;
mod config;
mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // Fill browser window
                    fit_canvas_to_parent: true,
                    // Allow for browser shortcuts
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<MultiplayerConfig>::default(),
            PhysicsPlugins::default(),
            LdtkPlugin,
        ))
        .rollback_component_with_clone::<Transform>()
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_systems(
            Startup,
            (
                systems::setup,
                systems::player::spawn,
                systems::multiplayer::start_matchbox_socket,
            ),
        )
        .add_systems(Update, (systems::multiplayer::wait_for_payers,))
        .add_systems(ReadInputs, systems::multiplayer::read_local_inputs)
        .add_systems(GgrsSchedule, systems::player::move_players)
        .insert_resource(LevelSelection::index(0))
        .run();
}
