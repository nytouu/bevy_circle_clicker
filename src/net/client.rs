use bevy::{prelude::*, window::PrimaryWindow};
use bevy_renet::{
    client_connected,
    renet::{transport::ClientAuthentication, ConnectionConfig, DefaultChannel, RenetClient},
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use renet::{transport::NetcodeClientTransport, ClientId};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

use crate::game::cursor::*;
use crate::net::*;
use crate::utils::*;

pub struct NetClientPlugin;

impl Plugin for NetClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RenetClientPlugin, NetcodeClientPlugin));
        app.init_resource::<CursorPosition>();
        let (server, transport) = new_renet_client();
        app.insert_resource(server).insert_resource(transport);

        app.add_systems(
            Update,
            (player_input, client_send_input, client_sync_players).run_if(client_connected),
        );
        app.add_systems(FixedUpdate, (spawn_trail, remove_trail).run_if(client_connected));
    }
}

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(ConnectionConfig::default());

    (client, transport)
}

fn client_sync_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    asset_server: Res<AssetServer>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected { id } => {
                println!("Player {} connected.", id);
                let player_entity = commands
                    .spawn((
                        Cursor,
                        SpriteBundle {
                            texture: asset_server.load("cursor.png"),
                            ..Default::default()
                        },
                    ))
                    .id();

                lobby.players.insert(id, player_entity);
            }
            ServerMessages::PlayerDisconnected { id } => {
                println!("Player {} disconnected.", id);
                if let Some(player_entity) = lobby.players.remove(&id) {
                    commands.entity(player_entity).despawn();
                }
            }
            ServerMessages::Click { position } => todo!(),
        }
    }

    // Update cursors positions
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let players: HashMap<ClientId, [f32; 3]> = bincode::deserialize(&message).unwrap();
        for (player_id, translation) in players.iter() {
            if let Some(player_entity) = lobby.players.get(player_id) {
                let transform = Transform {
                    translation: (*translation).into(),
                    ..Default::default()
                };
                commands.entity(*player_entity).insert(transform);
            }
        }
    }
}

fn client_send_input(player_cursor_position: Res<CursorPosition>, mut client: ResMut<RenetClient>) {
    let input_message = bincode::serialize(&*player_cursor_position).unwrap();

    client.send_message(DefaultChannel::ReliableOrdered, input_message);
}

fn player_input(
    mut player_position: ResMut<CursorPosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        player_position.0 = world_position;
    };
}
