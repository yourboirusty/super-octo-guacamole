use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsPlugin, ReadInputs};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use config::MultiplayerConfig;
use game::GameState;
use systems::{check_asset_loading, player::PlayerPlugin};

mod components;
mod config;
mod game;
mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // Fill browser window
                        fit_canvas_to_parent: true,
                        // Allow for browser shortcuts
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            LdtkPlugin,
            GgrsPlugin::<MultiplayerConfig>::default(),
            PhysicsPlugins::default(),
            PlayerPlugin,
            // EguiPlugin {},
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            set_clear_color: SetClearColor::No,
            ..Default::default()
        })
        .init_state::<GameState>()
        .rollback_component_with_clone::<Transform>()
        .add_systems(
            Startup,
            (systems::setup, systems::multiplayer::start_matchbox_socket),
        )
        .add_systems(Update, (systems::player::camera_fit_inside_current_level,))
        .add_systems(
            Update,
            check_asset_loading.run_if(in_state(GameState::Loading)),
        )
        .add_systems(
            Update,
            (systems::multiplayer::wait_for_payers.run_if(in_state(GameState::Playing)),),
        )
        .add_systems(ReadInputs, systems::multiplayer::read_local_inputs)
        .insert_resource(LevelSelection::index(0))
        .run();
}
