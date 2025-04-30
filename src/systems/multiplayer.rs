use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::{
    LocalInputs, LocalPlayers,
    ggrs::{self},
};
use bevy_matchbox::MatchboxSocket;

use crate::systems::{
    colliders::{CharacterCollider, ColliderBundle},
    player::SpawnPlayerEvent,
};
use crate::{
    config::*,
    systems::player::{Player, PlayerBundle},
};

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
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
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
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
        info!("Created player");
        let texture = asset_server.load("atlas/Player.png");
        let layout = texture_atlases.add(TextureAtlasLayout::from_grid(
            UVec2::new(32, 32),
            7,
            6,
            None,
            None,
        ));
        let atlas = TextureAtlas { index: 0, layout };
        let player_c = commands.spawn(PlayerBundle {
            player: Player { handle: i },
            sprite_sheet: Sprite::from_atlas_image(texture, atlas),
            collider_bundle: ColliderBundle::from(CharacterCollider::Player),
            ..Default::default()
        });

        spawned_event.send(SpawnPlayerEvent(player_c.id()));
    }

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
