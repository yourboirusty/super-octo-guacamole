use avian2d::{math::Scalar, prelude::*};
use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::{
    AddRollbackCommandExtension, LocalInputs, LocalPlayers,
    ggrs::{self},
    prelude::*,
};
use bevy_matchbox::MatchboxSocket;

use crate::{
    config::*,
    systems::{
        controller::CharacterControllerBundle,
        player::{Player, PlayerBundle},
    },
};
use crate::{
    game::GameState,
    systems::{colliders::CharacterCollider, player::SpawnPlayerEvent},
};

use super::controller::Grounded;

const TARGET_FPS: usize = 60;

#[derive(Component)]
pub struct Local;

pub fn start_matchbox_socket(mut commands: Commands) {
    // wasm_test -> scope
    // next=2 -> make room connect pairs as they connect
    let room_url = "ws://127.0.0.1:3536/wasm_test?next=2";
    info!("Connecting to matchbox server");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

pub fn wait_for_payers(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket>,
    mut spawned_event: EventWriter<SpawnPlayerEvent>,
    asset_server: Res<AssetServer>,
) {
    if socket.get_channel(0).is_err() {
        return;
    }
    socket.update_peers();
    let players = socket.players();
    let num_players = 2;
    if players.len() < num_players {
        return;
    }
    info!("All peers have joined, starting game");

    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<MultiplayerConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(5)
        .with_fps(TARGET_FPS)
        .unwrap()
        .with_max_prediction_window(8)
        .with_sparse_saving_mode(false)
        .with_desync_detection_mode(ggrs::DesyncDetection::On { interval: 1 });

    // Add local player handles - for simplicity, assume player 0 is local
    let mut local_player_handles = Vec::new();

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
        info!("Created player");
        let texture = match player {
            PlayerType::Local => {
                local_player_handles.push(i);
                asset_server.load("atlas/kornel.png")
            }
            _ => asset_server.load("atlas/wera.png"),
        };
        let sprite_sheet = Sprite::from_image(texture);

        let mut player_c = commands.spawn(PlayerBundle {
            player: Player { handle: i },
            sprite_sheet,
            character_controller: CharacterControllerBundle::from(CharacterCollider::Player),
            ..Default::default()
        });
        if player == PlayerType::Local {
            player_c.insert(Local);
        }

        spawned_event.send(SpawnPlayerEvent(player_c.id()));
        player_c.add_rollback();
    }

    // Add resource for local players
    commands.insert_resource(LocalPlayers(local_player_handles));
    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
}

pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input = 0u8;

        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            input |= INPUT_UP;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            input |= INPUT_DOWN;
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            input |= INPUT_RIGHT;
        }
        if keys.any_pressed([KeyCode::Space, KeyCode::Enter]) {
            input |= INPUT_FIRE;
        }
        if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ControlRight]) {
            input |= INPUT_USE;
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<MultiplayerConfig>(local_inputs));
}

pub struct MultiplayerPlugin;
impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GgrsPlugin::<MultiplayerConfig>::default())
            .rollback_component_with_clone::<Transform>()
            // .rollback_component_with_clone::<Rotation>()
            .rollback_component_with_clone::<GlobalTransform>()
            .rollback_component_with_clone::<LinearVelocity>()
            // .rollback_component_with_clone::<AngularVelocity>()
            .rollback_component_with_clone::<Position>()
            // .rollback_component_with_clone::<Sleeping>()
            .rollback_component_with_clone::<ShapeHits>()
            .rollback_component_with_clone::<Grounded>()
            // .rollback_component_with_clone::<TimeSleeping>()
            .rollback_component_with_clone::<CollidingEntities>()
            .rollback_component_with_clone::<Rotation>()
            .rollback_resource_with_clone::<Time<Physics>>()
            .rollback_resource_with_clone::<Time>()
            .rollback_resource_with_clone::<Collisions>()
            // .rollback_component_with_clone::<Grounded>()
            .checksum_component::<Position>(|position| {
                let mut bytes: Vec<u8> = Vec::new();
                bytes.extend(position.x.to_ne_bytes());
                bytes.extend(position.y.to_ne_bytes());
                fletcher16(&bytes) as u64
            })
            // .rollback_component_with_clone::<Collisions>()
            .set_rollback_schedule_fps(TARGET_FPS)
            .add_systems(Startup, start_matchbox_socket)
            .add_systems(ReadInputs, read_local_inputs)
            .add_systems(
                Update,
                (wait_for_payers.run_if(in_state(GameState::Playing)),),
            );
    }
}

pub fn fletcher16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;

    for byte in data {
        sum1 = (sum1 + *byte as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }

    (sum2 << 8) | sum1
}
