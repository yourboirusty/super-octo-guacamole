mod prelude {
    pub use avian2d::prelude::*;
    pub use bevy::prelude::*;
    pub use bevy_ecs_ldtk::prelude::*;
    pub use bevy_tnua::prelude::*;
}
use bevy_ggrs::GgrsSchedule;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;
use config::LEVEL_IIDS;
use game::GameState;
use systems::{
    check_asset_loading,
    controller::{ControllerPlugin, MovementEvent, process_inputs},
    frame_logging::{
        CurrentSessionFrame, RollbackStatus, update_current_session_frame, update_rollback_status,
    },
    multiplayer::MultiplayerPlugin,
    player::{PlayerPlugin, camera::camera_follow_local_players, movement::move_players},
};

mod components;
mod config;
mod game;
mod systems;

use crate::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
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
        PlayerPlugin,
        MultiplayerPlugin,
        ControllerPlugin,
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
    .insert_resource(RollbackStatus::default())
    .insert_resource(CurrentSessionFrame::default())
    .init_state::<GameState>()
    .add_systems(Startup, systems::setup)
    .add_systems(
        Update,
        (check_asset_loading.run_if(in_state(GameState::Loading)),),
    )
    .insert_resource(LevelSelection::Iid(LevelIid::new(LEVEL_IIDS[0])));

    app.get_schedule_mut(bevy_ggrs::GgrsSchedule)
        .unwrap()
        .set_build_settings(bevy::ecs::schedule::ScheduleBuildSettings::default());
    app.add_plugins(PhysicsPlugins::default().with_length_unit(12.0));
    app.add_plugins((
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian2dPlugin::new(FixedUpdate),
    ));

    app.add_event::<MovementEvent>();

    app.add_systems(
        Update,
        camera_follow_local_players.run_if(in_state(GameState::Playing)),
    );

    app.add_systems(
        bevy_ggrs::GgrsSchedule,
        (
            update_current_session_frame,
            update_rollback_status,
            process_inputs,
            apply_deferred,
        )
            .chain()
            .before(PhysicsSet::Prepare)
            .in_set(TnuaUserControlsSystemSet),
    );
    app.add_systems(FixedUpdate, move_players.in_set(TnuaUserControlsSystemSet));

    app.add_systems(
        bevy_ggrs::GgrsSchedule,
        (apply_deferred).chain().before(PhysicsSet::Sync),
    );

    app.run();
}
