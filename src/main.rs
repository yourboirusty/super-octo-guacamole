use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::GgrsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use config::{LEVEL_IIDS, MultiplayerConfig};
use game::GameState;
use systems::{
    check_asset_loading,
    multiplayer::MultiplayerPlugin,
    player::{PlayerControllerPlugin, PlayerPlugin},
};

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
            PhysicsPlugins::default().with_length_unit(12.0),
            PlayerPlugin,
            PlayerControllerPlugin,
            MultiplayerPlugin,
            WorldInspectorPlugin::new(),
            PhysicsDebugPlugin::default(),
            systems::walls::WallPlugin,
        ))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            set_clear_color: SetClearColor::No,
            ..Default::default()
        })
        .insert_resource(Gravity(Vec2::NEG_Y * 84.0))
        .init_state::<GameState>()
        .add_systems(Startup, systems::setup)
        .add_systems(Update, (systems::player::camera_fit_inside_current_level,))
        .add_systems(
            Update,
            check_asset_loading.run_if(in_state(GameState::Loading)),
        )
        .add_systems(
            Update,
            (systems::multiplayer::wait_for_payers.run_if(in_state(GameState::Playing)),),
        )
        .insert_resource(LevelSelection::Iid(LevelIid::new(LEVEL_IIDS[0])))
        .run();
}
